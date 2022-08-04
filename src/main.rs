use std::env;
use std::fmt::Display;

use rand::Rng;
use serenity::prelude::*;
use serenity::{
    framework::{
        StandardFramework,
        standard::{
            macros::{
                command,
                group,
            },
            CommandResult,
            Args,
        },
    },
    model::{
        channel::Message,
        gateway::Ready,
    },
    Client,
    async_trait,
};

#[group]
#[commands(ping, rps)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _ready: Ready) {
        println!("Bot logged in");
    }

    // async fn message(&self, _ctx: Context, msg: Message) {
    //     if msg.author.bot {
    //         return
    //     }
    // }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("TOKEN").expect("Invalid token");
    let prefix = env::var("PREFIX").unwrap_or("!".to_string());

    let framework = StandardFramework::new()
        .configure(|s| {
            s
                .ignore_bots(true)
                .prefix(prefix)
        })
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::all();
    let mut client = Client::builder(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .unwrap();

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "Pong!").await;
    Ok(())
}

#[command]
async fn rps(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let outcomes: Vec<String> = ["rock", "paper", "scissors"].map(|x| x.to_string()).to_vec();

    let bot_outcome_index = rand::thread_rng().gen_range(0..outcomes.len());
    let bot_outcome = &outcomes[bot_outcome_index].to_string();

    let user_input = args.single::<String>();
    let user_outcome = user_input.unwrap_or("invalid".to_string());

    if !outcomes.contains(&user_outcome) {
        reply_message(ctx, msg, "Invalid input").await;
        return Ok(());
    }

    if bot_outcome.to_string() == user_outcome.to_string() {
        match msg.channel_id.send_message(ctx, |m| {
            m.reference_message(msg)
            .content(format!("I got {}! It's a tie.", bot_outcome.to_string()))
        }).await {
            Ok(_) => {
                return Ok(());
            },
            Err(e) => println!("{}", e)
        };
    }

    match bot_outcome.as_str() {
        "rock" => {
            match user_outcome.as_str() {
                "paper" => {
                    reply_message(ctx, msg, "I got rock. I win!").await;
                }
                "scissor" => {
                    reply_message(ctx, msg, "I got rock. I lose!").await;
                }
                _ => tie(ctx, msg, bot_outcome).await
            }
        }
        "paper" => {
            match user_outcome.as_str() {
                "rock" => {
                    reply_message(ctx, msg, "I got paper! I win!").await;
                }
                "scissor" => {
                    reply_message(ctx, msg, "I got paper! I lose!").await;
                }
                _ => tie(ctx, msg, bot_outcome).await
            }
        }
        "scissors" => {
            match user_outcome.as_str() {
                "paper" => {
                    reply_message(ctx, msg, "I got scissors! I win!").await;
                }
                "rock" => {
                    reply_message(ctx, msg, "I got scissors! I lose").await;
                }
                _ => tie(ctx, msg, bot_outcome).await
            }
        }
        _ => {}
    }
    
    Ok(())
}

async fn reply_message(ctx: &Context, msg: &Message, content: impl Display) {
    match msg.reply(ctx, content).await {
        Ok(_) => {},
        Err(e) => {
            println!("An error occured while replying to a message: {}", e)
        }
    };
}

async fn tie(ctx: &Context, msg: &Message, bot_outcome: &str) {
    reply_message(ctx, msg, format!("I got {}! We drew!", bot_outcome)).await;
}
