import {Bot } from "grammy"
import { BOT_TOKEN } from "./configs/config"
import { ContestController } from "./contest"

/*
  Dank My Meme Telegram Bot
  - add bot to channel
  - start contest by admin
  - 
*/

if(BOT_TOKEN === undefined) process.exit(".env errror");

const cc = new ContestController()

const bot = new Bot(BOT_TOKEN)

function replyStrartContest(){
  return `please go to: https://www.dankmymeme.xyz to start a contest`
}


bot.command('start', (ctx) => {

  console.log(ctx.channelPost);

  const chatId = ctx.chatId;

  if(ctx.channelPost?.message_id){
    if(chatId in cc.channels) ctx.reply("You already started me dumbass");
    else cc.addChannel(chatId.toString())
  }
  else ctx.reply('gay')
  //admin only
  //save channel id
  //start 24 hour countdown for submissions
  // reply address for prizepool
  // 24 hours to add prizepool

})


bot.command('startcontest', (ctx) => {

  console.log(ctx.channelPost)

  const chatId = ctx.chatId;

  const formData = {
    tokenAddress: "",
    name: ctx.chat.title!,
    startDateTime: "",
    endDateTime: "",
    entryFee: "",
    votingFee: "",
    winnerPercentage: "0",
    numberOfLuckyVoters: "0"
  }

  if(ctx.channelPost?.message_id){
    cc.channels[chatId].createContest(formData)
  }
  else ctx.reply('big gay')
  //admin only
  //save channel id
  //start 24 hour countdown for submissions
  // reply address for prizepool
  // 24 hours to add prizepool

})

bot.command('submit', (ctx) => {
  ctx.channelPost?.message_id? ctx.reply('meme submitted') : ctx.reply('big gay')
  //any user - need to be aproved by admin?
  //save tg user id
  //pic - meme
  //add to contest
  //

})

bot.on('message', (ctx) => {
  ctx.reply('helloooo')
})

bot.start()

//-24 hour countdown-
//class? 
//save channel id
// prizepool
//after 24 hours reply with message 
//- username - meme - poll
//set another 24 hour timer where users can select the message to vote on best meme
