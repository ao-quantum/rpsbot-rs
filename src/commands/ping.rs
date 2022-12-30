use serenity::prelude::Context;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

pub async fn run(ctx: Context, interaction: ApplicationCommandInteraction) {
    match interaction.create_interaction_response(ctx,|r|
        r
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d|
                d
                    .ephemeral(true)
                    .content("Pong!")
            )).await {
        Ok(()) => {},
        Err(e) => {
            println!("Error! Tried to respond to interaction but failed: {:?}", e)
        },
    };
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("Ping the bot")
}
