use serenity::builder::{CreateEmbed};
use serenity::client::Context;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;

pub async fn reply_cmd_interaction(
    ctx: Context,
    interaction: ApplicationCommandInteraction,
    ephemeral: bool,
    embeds: Vec<CreateEmbed>,
    content: Option<&str>
) -> serenity::Result<()> {
    return interaction.create_interaction_response(ctx, |r|
        r
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|d| {
                d
                    .add_embeds(embeds)
                    .ephemeral(ephemeral);

                if content.is_some() {
                    d.content(content.unwrap());
                }

                return d;
            })
    ).await;
}
