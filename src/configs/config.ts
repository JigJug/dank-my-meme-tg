import dotenv from "dotenv"
import ethers from "ethers"
import * as fs from "fs"
dotenv.config()

export const BOT_TOKEN = process.env.BOT_TOKEN

export const PROVIDER = new ethers.JsonRpcProvider(process.env.PROVIDER_URL)

export const ABI = JSON.parse(fs.readFileSync("./contest.json").toString())