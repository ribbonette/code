use ribbon_commands_core::{ Context, Interaction };
use ribbon_syncing::SyncOperation;
use ribbon_util::DISCORD_INTERACTION_CLIENT;
use twilight_model::{
	application::interaction::{
		application_command::CommandOptionValue,
		Interaction as TwilightInteraction, InteractionType
	},
	http::interaction::{ InteractionResponse, InteractionResponseData, InteractionResponseType }
};

use crate::Result;

async fn parse_interaction(context: Context) -> Result<()> {
	if let Some((context, command)) = ribbon_commands::commands::process_context(context) {
		for option in context.options.iter() {
			if let CommandOptionValue::Focused(partial, _kind) = &option.value {
				let interaction_id = context.interaction.id;
				let interaction_token = context.interaction.token.clone();

				let partial = partial.clone();
				let command_option = command.options
					.iter()
					.find(|x| x.name == option.name)
					.unwrap();
				let choices = command_option.autocomplete.unwrap()(context, partial).await?;
				DISCORD_INTERACTION_CLIENT.create_response(interaction_id, &interaction_token, &InteractionResponse {
					kind: InteractionResponseType::ApplicationCommandAutocompleteResult,
					data: Some(InteractionResponseData {
						choices: Some(choices),
						..Default::default()
					})
				}).await?;

				return Ok(());
			}
		}

		match (command.handler)(context).await {
			Ok(x) => x,
			Err(error) => {
				println!("{error}");
				return Err(error.into());
			}
		}
	} else {
		println!("command no tfoun");
	}

	Ok(())
}

pub async fn handle_interaction(interaction: TwilightInteraction) -> Result<()> {
	if matches!(interaction.kind, InteractionType::MessageComponent) {
		SyncOperation::from_interaction(interaction, false)
			.await?;
		return Ok(());
	}

	let interaction = Interaction {
		app_permissions: interaction.app_permissions,
		application_id: interaction.application_id,
		channel: interaction.channel,
		data: interaction.data,
		guild_id: interaction.guild_id,
		guild_locale: interaction.guild_locale,
		id: interaction.id,
		kind: interaction.kind,
		locale: interaction.locale,
		message: interaction.message,
		token: interaction.token,
		user_id: match interaction.member {
			Some(member) => member.user.map(|x| x.id),
			None => interaction.user.map(|x| x.id)
		}
	};

	parse_interaction(Context::new(interaction))
		.await?;

	Ok(())
}