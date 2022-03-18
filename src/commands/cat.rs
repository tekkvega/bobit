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
use serenity::framework::standard::{Args, Delimiter};

#[command]
#[description("Send a picture of a cat :3")]
#[help_available(true)]
async fn cat(ctx: &Context, msg: &Message) -> CommandResult{
    let mut link = "https://cataas.com/cat".to_string();
    let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
    if args.len() >= 2 {
        args.single::<String>().unwrap();
        link = link + "/says/" + &args.rest().replace(" ", "%20");
    }

    let img_bytes = reqwest::get(link)
     .await?
     .bytes();
    image::load_from_memory(&img_bytes.await.unwrap().to_vec())?.save(Path::new("./cat.jpg"));

    msg.channel_id.send_message(&ctx.http, |m| {
        m.content("cute cat for you :3").add_file("cat.jpg")
    }).await;
    fs::remove_file("cat.jpg")?;
    Ok(())
}
