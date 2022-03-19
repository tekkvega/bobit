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
use serenity::model::channel::Embed;
use serenity::utils::Colour;
use serenity::http::CacheHttp;


#[command]
#[description("Shows all commands")]
#[help_available(true)]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg
    .author.dm(&ctx.http, |m| {
        m.content("Here are all the commands for you :)")
        .embed(|e| {
            e.title("Help")
                .colour(Colour::from_rgb(0, 251, 255))
                .description("All commands available");
                for command in COMMANDS_GROUP_OPTIONS.commands.to_vec().iter() {
                    let desc = command.options.desc;
                    let name = command.options.names;
                    e.field(name.to_vec()[0].to_string(), desc.unwrap().to_string(), true);
                }
                e.field("ping", "pong! (no prefix)", true);
                e.footer(|f| f.text("By Mott's Applesauce"))
            })
    })
    .await;
    msg.channel_id.say(&ctx.http, format!("Command list sent to user: {}", msg.author.to_string())).await?;
    Ok(())
}

#[group]
#[commands(test, help, say, rps, cat, flag, meme)]
struct Commands;
