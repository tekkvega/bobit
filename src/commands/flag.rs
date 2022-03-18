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
use std::fs;
use rand::Rng;
use serenity::framework::standard::{Args, Delimiter};
use serde_json::{Result, Value};

#[command]
#[description("Nonexistant flags")]
#[help_available(true)]
async fn flag(ctx: &Context, msg: &Message) -> CommandResult{
    let ranNum: i16 = rand::thread_rng().gen_range(0..4999);
    let bytes = reqwest::get(format!("https://thisflagdoesnotexist.com/images/{}.png", ranNum)).await?;
    let json = reqwest::get(format!("https://thisflagdoesnotexist.com/flags/{}.json", ranNum)).await?.text().await?;
    let v: Value = serde_json::from_str(&json)?;
    let name = v["name"].as_str().unwrap();
    image::load_from_memory(&bytes.bytes().await.unwrap().to_vec())?.save(Path::new("./flag.jpg"));
    msg.channel_id.send_message(&ctx.http, |m| {
        m.content(format!("this flag doesn't exist, this is flag #{}\nThe Flag of {}", ranNum, name)).add_file("flag.jpg")
    }).await;
    fs::remove_file("flag.jpg")?;
    Ok(())
}
