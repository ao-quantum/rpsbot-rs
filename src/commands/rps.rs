use rand::Rng;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::interactions::application_command::ApplicationCommandOptionType;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use crate::OPTIONS;
use crate::util::reply_cmd_interaction::reply_cmd_interaction;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("rps")
        .description("Play rock paper scissor")
        .create_option(|o|
            o
                .name("choice")
                .description("Pick rock, paper, or scissor")
                .kind(ApplicationCommandOptionType::String)
                .required(true)
                .add_string_choice("rock", "rock")
                .add_string_choice("paper", "paper")
                .add_string_choice("scissor", "scissor")
        )
}

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) {
    let bot_outcome_index = rand::thread_rng().gen_range(0..OPTIONS.len());
    let bot_outcome = &OPTIONS[bot_outcome_index].to_string();

    let user_input = interaction.data.options.get(0).unwrap();
    let user_outcome = match user_input.value {
        Some(ref value) => value.as_str().unwrap(),
        None => "invalid",
    };

    println!("User outcome: {}", user_outcome);

    if !OPTIONS.contains(&user_outcome) {

        reply_cmd_interaction(ctx, interaction, true, vec![], Some("Invalid Input"))
            .await
            .expect("Could not send RPS result message");
        return;
    }

    if bot_outcome.to_string() == user_outcome.to_string() {
        reply_cmd_interaction(ctx, interaction, true, vec![], Some("It's a tie!"))
            .await
            .expect("Could not send RPS result message");
        return;
    }

    // Match comments are formatted as "bot_outcome vs user_outcome"
    match bot_outcome.as_str() {
        "rock" => {
            match user_outcome {
                "paper" => {
                    // rock vs paper
                    reply_cmd_interaction(ctx, interaction, true, vec![], Some("I got rock. I lose!"))
                        .await
                        .expect("Could not send RPS result message");
                }
                "scissor" => {
                    // rock vs scissor
                    reply_cmd_interaction(ctx, interaction, true, vec![], Some("I got rock. I win!"))
                        .await
                        .expect("Could not send RPS result message");
                }
                _ => tie(ctx, interaction, bot_outcome).await
            }
        }
        "paper" => {
            match user_outcome {
                "rock" => {
                    // paper vs rock
                    reply_cmd_interaction(ctx, interaction, true, vec![], Some("I got paper. I win!"))
                        .await
                        .expect("Could not send RPS result message");
                }
                "scissor" => {
                    // paper vs scissor
                    reply_cmd_interaction(ctx, interaction, true, vec![], Some("I got paper. I lose!"))
                        .await
                        .expect("Could not send RPS result message");
                }
                _ => tie(ctx, interaction, bot_outcome).await
            }
        }
        "scissors" => {
            match user_outcome {
                "paper" => {
                    // scissors vs paper
                    reply_cmd_interaction(ctx, interaction, true, vec![], Some("I got scissors. I win!"))
                        .await
                        .expect("Could not send rock paper scissor result message");
                }
                "rock" => {
                    // scissors vs rock
                    reply_cmd_interaction(ctx, interaction, true, vec![], Some("I got scissors. I lose!"))
                        .await
                        .expect("Could not send rock paper scissor result message");
                }
                _ => tie(ctx, interaction, bot_outcome).await
            }
        }
        _ => {}
    }
}

async fn tie(ctx: Context, interaction: ApplicationCommandInteraction, bot_outcome: &str) {
    match reply_cmd_interaction(ctx, interaction, true, vec![], Some(&*format!("I got {}. We drew!", bot_outcome))).await {
        Ok(_) => {}
        Err(e) => {
            println!("Error sending tie message: {}", e);
        }
    };
}
