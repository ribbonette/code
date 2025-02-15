use twilight_model::gateway::payload::incoming::InteractionCreate;

use crate::{
	discord::interactions::handle_interaction,
	Result
};

pub async fn interaction_create(interaction_create: InteractionCreate) -> Result<()> {
	handle_interaction(interaction_create.0).await
}