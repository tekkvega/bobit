#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::CommandResult;
use serenity::model::user::User;
use std::vec;
use serenity::framework::standard::{Args, Delimiter};
use serenity::http::CacheHttp;
use rand::Rng;
use std::sync::Mutex;

/*
ROCK     - SCISSORS = 0 - 2 = -2 -> (-2 + 3) % 3 = WIN
PAPER    - SCISSORS = 1 - 2 = -1 -> (-1 + 3) % 3 = LOSE
ROCK     - PAPER    = 0 - 1 = -1 -> (-1 + 3) % 3 = LOSE
PAPER    - ROCK     = 1 - 0 =  1 -> ( 1 + 3) % 3 = WIN
SCISSORS - PAPER    = 2 - 1 =  1 -> ( 1 + 3) % 3 = WIN
SCISSORS - ROCK     = 2 - 0 =  2 -> ( 2 + 3) % 3 = LOSE
*/
#[command]
#[description("Rock, Paper, Scissors, SHOOT! use rock (:fist:), paper (:raised_hand:) or, scissors (:v:) to try to win")]
#[help_available(true)]
async fn rps(ctx: &Context, msg: &Message) -> CommandResult{
    let mut choice: i8 = 3;
    let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
    args.single::<String>().unwrap();
    if args.rest() == "rock" || args.rest() == "✊" {
        choice = 0;
    }
    else if args.rest() == "paper" || args.rest() == "✋" {
        choice = 1;
    }
    else if args.rest() == "scissors" || args.rest() == "✌️" {
        choice = 2;
    }
    else {
        msg.channel_id.say(&ctx.http, "That input is not valid, please retry :)").await;

    }

    if choice == 0 ||choice == 1 || choice == 2 {
        let botChoice: i8 = rand::thread_rng().gen_range(0..2);
        match botChoice {
            0 => msg.reply(&ctx.http, ":fist:"),
            1 => msg.reply(&ctx.http, ":raised_hand:"),
            _ => msg.reply(&ctx.http, ":v:")
        };
        match (choice - botChoice + 3) % 3 {
            1 => msg.reply(&ctx.http, "I win :) you lost!"),
            0 => msg.reply(&ctx.http, ":| its a tie"),
            _ => msg.reply(&ctx.http, ":( I lost, you win")
        };
    }


    Ok(())
}
