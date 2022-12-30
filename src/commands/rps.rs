use rand::Rng;
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::client::Context;
use serenity::model::application::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
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
                .kind(CommandOptionType::String)
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
    let user_outcome_index = OPTIONS.iter().position(|&r| r == user_outcome);

    println!("User outcome: {}", user_outcome);

    if user_outcome_index.is_none() {
        reply_cmd_interaction(ctx, interaction, true, vec![], Some("Invalid Input"))
            .await
            .expect("Could not send RPS result message");
        return;
    }

    let user_outcome_index = user_outcome_index.unwrap();

    let result: &str;

    if (user_outcome_index + 1) % 3 == bot_outcome_index {
        // Bot won
        result = "I won!";
    } else if (bot_outcome_index + 1) % 3 == user_outcome_index {
        // Player won
        result = "You won!";
    } else {
        // Tie
        result = "We drew!";
    }

    reply_cmd_interaction(
        ctx,
        interaction.to_owned(),
        true,
        vec![
            CreateEmbed::default()
                .title("Rock Paper Scissors")
                .description(
                    format!("You: {}\n\
                    Me: {}\n\
                    {}",
                            user_outcome,
                            bot_outcome,
                            result
                    )
                )
                .to_owned()
        ],
        None)
        .await
        .expect("Could not send RPS result message");
}
