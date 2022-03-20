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
use songbird::SerenityInit;
use serenity::client::bridge::gateway::GatewayIntents;
use crate::commands::help::COMMANDS_GROUP_OPTIONS;
use serenity::utils::Colour;
use serenity::client::TokenComponents;
use serenity::model::prelude::Activity;

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
        if msg.mentions.len() > 0 && &msg.mentions[0].id == &ctx.http.get_current_user().await.unwrap().id {
            msg
            .author.dm(&ctx.http, |m| {
                m.content("Here are all the commands for you :)")
                .embed(|e| {
                    e.title("Help\nMy prefix is \"&\"")
                        .colour(Colour::from_rgb(0, 251, 255))
                        .description("All commands available");
                        for command in COMMANDS_GROUP_OPTIONS.commands.to_vec().iter() {
                            let desc = command.options.desc;
                            let name = command.options.names;
                            e.field(name.to_vec()[0].to_string(), desc.unwrap().to_string(), true);
                        }
                        e.field("ping", "pong! (no prefix)", true);
                        e.field("help music", "Shows all music commands", true);
                        e.footer(|f| f.text("By Mott's Applesauce"))
                    })
            })
            .await;
            msg.channel_id.say(&ctx.http, format!("Command list sent to user: {}", msg.author.to_string())).await;
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let filestring = fs::read_to_string("bobit.json");
        let out: Value = serde_json::from_str(&filestring.unwrap()).unwrap();
        &ctx.set_activity(Activity::playing(format!("Prefix: {}", out["prefix"].as_str().unwrap()))).await;
    }

}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
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
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .intents(GatewayIntents::DIRECT_MESSAGES | GatewayIntents::DIRECT_MESSAGE_REACTIONS | GatewayIntents::DIRECT_MESSAGE_TYPING | GatewayIntents::GUILDS | GatewayIntents::GUILD_BANS | GatewayIntents::GUILD_EMOJIS | GatewayIntents::GUILD_INTEGRATIONS | GatewayIntents::GUILD_INVITES | GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MESSAGE_REACTIONS | GatewayIntents::GUILD_MESSAGE_TYPING | GatewayIntents::GUILD_PRESENCES | GatewayIntents::GUILD_SCHEDULED_EVENTS | GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::GUILD_WEBHOOKS)
        .register_songbird()
        .await
        .expect("Err creating client");

    tokio::spawn(async move {
        let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
    });

    tokio::signal::ctrl_c().await;
    println!("Received Ctrl-C, shutting down.");
}
