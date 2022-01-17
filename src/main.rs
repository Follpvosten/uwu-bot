use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use owoify_rs::{Owoifiable, OwoifyLevel::*};
use tbot::{
    contexts::methods::Message,
    types::{
        inline_query::{self, result::Article},
        input_message_content::Text,
        message::Kind,
        Message as Msg,
    },
};
use tracing::{error, info};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    info!("Creating bot");
    let mut bot = tbot::Bot::from_env("BOT_TOKEN").event_loop();
    info!("Fetching username");
    bot.fetch_username().await.unwrap();
    info!("Registering handlers");
    bot.inline(|ctx| async move {
        if ctx.query.is_empty() {
            if let Err(e) = ctx.answer([]).call().await {
                error!("Error answering inline query: {:?}", e);
            }
            return;
        }
        let results: Vec<_> = [Owo, Uwu, Uvu]
            .into_iter()
            .map(|level| {
                let uwu = ctx.query.owoify(&level);
                let title = match level {
                    Owo => "owo",
                    Uwu => "uwu",
                    Uvu => "uvu",
                };
                let id = {
                    let mut hasher = DefaultHasher::new();
                    hasher.write(uwu.as_bytes());
                    format!("{}-{}", title, hasher.finish())
                };
                let title = format!("owo level: {}", title);
                inline_query::Result::new(
                    id,
                    Article::new(title, Text::new(uwu.clone())).description(uwu),
                )
            })
            .collect();
        if let Err(e) = ctx.answer(results).call().await {
            error!("Error answering inline query: {:?}", e);
        }
    });
    bot.commands(["owo", "uwu", "uvu"], |ctx| async move {
        let (source_msg, id) = if let Some(Msg {
            id,
            kind: Kind::Text(txt),
            ..
        }) = &ctx.reply_to
        {
            (&txt.value, *id)
        } else {
            (&ctx.text.value, ctx.message_id)
        };
        if source_msg.is_empty() {
            return;
        }
        let level = match ctx.command.as_str() {
            "uwu" => Uwu,
            "uvu" => Uvu,
            _ => Owo,
        };
        let uwu = source_msg.owoify(&level);
        if let Err(e) = ctx.send_message(uwu).in_reply_to(id).call().await {
            error!("Error sending message: {:?}", e);
        }
    });

    info!("Starting up bot");
    tokio::select! {
        res = bot.polling().start() => { res.unwrap(); }
        _ = tokio::signal::ctrl_c() => { info!("Ctrl-C received"); }
    };
}
