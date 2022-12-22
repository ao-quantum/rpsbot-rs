mod util;
mod commands;

use std::env;
use std::fmt::Display;

use rand::Rng;
use serenity::model::prelude::*;
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

use crate::util::reply_cmd_interaction::reply_cmd_interaction;

const OPTIONS: [&str; 3] = ["rock", "paper", "scissor"];

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    
    /**
     * ------------------------------------
     * Ready
     * ------------------------------------
    */
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("Bot logged in");

        // Register slash commands with discord
        match serenity::model::application::command::Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|c| commands::ping::register(c))
                .create_application_command(|c| commands::rps::register(c))
        }).await {
            Ok(_) => println!("Successfully registered slash commands"),
            Err(e) => println!("Error registering slash commands: {}", e),
        };
    }

    /**
     * ------------------------------------
     * Interaction create
     * ------------------------------------
     */
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received application command: {:#?}", command.data.name);

            match command.data.name.as_str() {
                "ping" => commands::ping::run(ctx, command).await,
                "rps" => commands::rps::run(ctx, command).await,
                _ => {
                    reply_cmd_interaction(ctx, command, true, vec![], Some("Invalid command"))
                        .await
                        .expect("Could not send invalid command message");
                },
            }
        }

    }
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
        });

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
