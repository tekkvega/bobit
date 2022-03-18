#![allow(unused_imports)]
#![allow(dead_code)]
#[allow(unused_must_use)]
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::{Args, Delimiter};
use serenity::http::CacheHttp;


#[command]
#[description("repeats what you say back to you")]
#[help_available(true)]
async fn say(ctx: &Context, msg: &Message) -> CommandResult {
    let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
    args.single::<String>().unwrap();
    msg.channel_id.say(&ctx.http, args.rest()).await?;
    msg.delete(&ctx.http).await;
    Ok(())
}
