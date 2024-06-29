import {Bot } from "grammy"
import dotenv from "dotenv"

/*
  Dank My Meme Telegram Bot
  - add bot to channel
  - start contest by admin
  - 
*/

dotenv.config()

const BOT_TOKEN = process.env.BOT_TOKEN
if(BOT_TOKEN === undefined) process.exit(".env errror")

const bot = new Bot(BOT_TOKEN)


bot.command('start-contest', (ctx) => {
  console.log(ctx.channelPost)
  ctx.channelPost?.message_id? ctx.reply('contest started') : ctx.reply('gay')
  //admin only
  //save channel id
  //start 24 hour countdown for submissions
  // reply address for prizepool
  // 24 hours to add prizepool

})

bot.command('submit', (ctx) => {
  //any user - need to be aproved by admin?
  //save tg user id
  //pic - meme
  //add to contest
  //
  
})

//-24 hour countdown-
//class? 
//save channel id
// prizepool
//after 24 hours reply with message 
//- username - meme - poll
//set another 24 hour timer where users can select the message to vote on best meme
