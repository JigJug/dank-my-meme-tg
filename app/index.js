"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const grammy_1 = require("grammy");
const dotenv_1 = __importDefault(require("dotenv"));
/*
  Dank My Meme Telegram Bot
  - add bot to channel
  - start contest by admin
  -
*/
dotenv_1.default.config();
const BOT_TOKEN = process.env.BOT_TOKEN;
if (BOT_TOKEN === undefined)
    process.exit(".env errror");
const bot = new grammy_1.Bot(BOT_TOKEN);
bot.command('startcontest', (ctx) => {
    var _a;
    console.log(ctx.channelPost);
    ((_a = ctx.channelPost) === null || _a === void 0 ? void 0 : _a.message_id) ? ctx.reply('contest started') : ctx.reply('gay');
    //admin only
    //save channel id
    //start 24 hour countdown for submissions
    // reply address for prizepool
    // 24 hours to add prizepool
});
bot.command('submit', (ctx) => {
    ctx.reply('submitted');
    //any user - need to be aproved by admin?
    //save tg user id
    //pic - meme
    //add to contest
    //
});
bot.on('message', (ctx) => {
    ctx.reply('helloooo');
});
bot.start();
//-24 hour countdown-
//class? 
//save channel id
// prizepool
//after 24 hours reply with message 
//- username - meme - poll
//set another 24 hour timer where users can select the message to vote on best meme
