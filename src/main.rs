#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(dead_code)]
use std::env;
pub mod commands;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use crate::commands::help::COMMANDS_GROUP;
use serenity::{async_trait,model::{channel::Message, gateway::Ready},prelude::*};
use std::collections::HashSet;
use serenity::http::Http;
use crate::commands::Handler;
use std::path::Path;
use std::ffi::OsStr;
use std::fs;
use serde::Deserialize;
use std::error::Error;
use std::io::BufReader;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct Jstruct {
    fingerprint: String,
    location: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "pong").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

}
#[tokio::main]
async fn main() {
    let filestring = fs::read_to_string("bobit.json");
    let out: Value = serde_json::from_str(&filestring.unwrap()).unwrap();
    let token = out["token"].as_str().unwrap();
    let http = Http::new_with_token(&token);
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };
    let framework = StandardFramework::new().configure(|c| c.owners(owners).prefix(out["prefix"].as_str().unwrap())).group(&COMMANDS_GROUP);
    let mut client = Client::builder(&token).framework(framework).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await
        {println!("Client error: {:?}", why);}

}
