#![allow(unused_imports)]
#![allow(dead_code)]
#[allow(unused_must_use)]
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::CommandResult;
use serenity::http::CacheHttp;


#[command]
#[description("just a test")]
#[help_available(true)]
async fn test(ctx: &Context, msg: &Message) -> CommandResult{
    msg.reply(&ctx.http, "test").await;

    Ok(())
}
