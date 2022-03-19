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
use std::path::Path;
use std::ffi::OsStr;
use serenity::model::channel::Embed;
use serenity::utils::Colour;
use std::fs;
use serde_json::Value;
use std::str;
use std::iter::Take;
use std::str::Chars;
use std::iter::Rev;
#[command]
#[description("Sends a random meme")]
#[help_available(true)]
async fn meme(ctx: &Context, msg: &Message) -> CommandResult{
    let text_bytes = reqwest::get("https://meme-api.herokuapp.com/gimme").await?.text().await?;
    let v: Value = serde_json::from_str(&text_bytes)?;
    let title = v["title"].as_str().unwrap();
    let postLink = v["postLink"].as_str().unwrap();
    let subreddit = v["subreddit"].as_str().unwrap();
    let url = v["url"].as_str().unwrap();
    msg.channel_id.send_message(&ctx.http, |m| {
        m.content(format!("Here's your meme!\n<{}>\n posted in: {}\n**{}**\n{}", postLink, subreddit, title, url))
    }).await;
    fs::remove_file("meme.jpg")?;

    Ok(())
}
