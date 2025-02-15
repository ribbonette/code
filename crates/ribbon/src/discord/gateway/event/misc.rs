use twilight_model::gateway::payload::incoming::Ready;

use crate::Result;

pub async fn ready(_ready: Ready) -> Result<()> {
	ribbon_emojis::load_all()
		.await?;

	Ok(())
}