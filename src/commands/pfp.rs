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
use serenity::http::CacheHttp;
use serenity::framework::standard::{Args, Delimiter};
use serenity::utils::Colour;

#[command]
#[description("sends the message authors or mentioned persons profile picture")]
#[help_available(true)]
async fn pfp(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    args.single::<String>();
    if msg.mentions.len() == 1 {

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("Here is {}'s pfp", msg.mentions[0].name))
                    .colour(Colour::from_rgb(0, 251, 255))
                    .image(msg.mentions[0].avatar_url().unwrap())
            })

        }).await;
    }
    else {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(format!("Heres your pfp, {}", msg.author.name))
                    .colour(Colour::from_rgb(0, 251, 255))
                    .image(msg.author.avatar_url().unwrap())
                })

        }).await;
    }


    Ok(())
}
