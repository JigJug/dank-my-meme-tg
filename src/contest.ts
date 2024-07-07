import ethers, { HDNodeWallet } from "ethers"
import { ABI, PROVIDER } from "./configs/config"

interface FormData{
  tokenAddress: string
  name: string
  startDateTime: string
  endDateTime: string
  entryFee: string
  votingFee: string
  winnerPercentage: string
  numberOfLuckyVoters: string
}

interface Channels{
  [key: string]: ChannelContests
}


function timestamp(time: string){
  return Math.floor(new Date(time).getTime() / 1000)
}


export class ContestController{
  channels: Channels
  constructor(){
    this.channels = {}
  }

  addChannel(channelId: string){
    const channelContest = new ChannelContests(channelId)
    this.channels[channelId] = channelContest
  }

}


class ChannelContests{
  channelId: string;
  wallet: HDNodeWallet;
  contests: string[];

  constructor(channelId: string){
    this.channelId = channelId;
    this.wallet = ethers.Wallet.createRandom(PROVIDER);
    this.contests = [];
  }

  async createContest(formData: FormData){
    const {
      tokenAddress,
      name,
      startDateTime,
      endDateTime,
      entryFee,
      votingFee,
      winnerPercentage,
      numberOfLuckyVoters
    } = formData;

    const contractFactory = new ethers.ContractFactory(ABI.abi, ABI.bytecode, this.wallet);

    try {
      const contract = await contractFactory.deploy(
          tokenAddress,
          name,
          timestamp(startDateTime),
          timestamp(endDateTime),
          entryFee,
          votingFee,
          winnerPercentage,
          numberOfLuckyVoters
      );

      const deployed = await contract.getDeployedCode();
      if (deployed === null) throw new Error("contract not deployed");

      this.contests.push(await contract.getAddress())

    } catch (err) {
      throw err
    }

  }
}