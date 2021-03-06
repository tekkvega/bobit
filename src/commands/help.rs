#![allow(unused_imports)]
#![allow(dead_code)]
#[allow(unused_must_use)]
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::CommandResult;
use crate::commands::test::TEST_COMMAND;
use crate::commands::say::SAY_COMMAND;
use crate::commands::rps::RPS_COMMAND;
use crate::commands::cat::CAT_COMMAND;
use crate::commands::flag::FLAG_COMMAND;
use crate::commands::meme::MEME_COMMAND;
use crate::commands::pfp::PFP_COMMAND;
use crate::commands::music::*;
use serenity::model::channel::Embed;
use serenity::utils::Colour;
use serenity::http::CacheHttp;
use serenity::framework::standard::{Args, Delimiter};


#[command]
#[description("Shows all commands")]
#[help_available(true)]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let mut args = Args::new(&msg.content, &[Delimiter::Single(' ')]);
    if args.len() >= 2 {
        args.single::<String>().unwrap();
        if args.rest() == "music" {
            msg
            .author.dm(&ctx.http, |m| {
                m.content("Here are all the music commands for you :)")
                .embed(|e| {
                    e.title("Music help\nMy prefix is \"&\"")
                        .colour(Colour::from_rgb(0, 251, 255))
                        .description("All music commands available\nOnly works in servers");
                        for command in MUSIC_GROUP_OPTIONS.commands.to_vec().iter() {
                            let desc = command.options.desc;
                            let name = command.options.names;
                            e.field(name.to_vec()[0].to_string(), desc.unwrap().to_string(), true);
                        }
                        e.footer(|f| f.text("By Mott's Applesauce"))
                    })
            })
            .await;
            msg.channel_id.say(&ctx.http, format!("Music command list sent to user: {}", msg.author.to_string())).await?;

        }
    }
    else {
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
        msg.channel_id.say(&ctx.http, format!("Command list sent to user: {}", msg.author.to_string())).await?;
    }
    Ok(())
}

#[group]
#[commands(test, help, say, rps, cat, flag, meme, pfp)]
struct Commands;

#[group]
#[commands(deafen, join, leave, mute, play, queue, skip, stop, undeafen, unmute)]
struct Music;
