use base64::{ prelude::BASE64_STANDARD, Engine };
use log::info;
use once_cell::sync::Lazy;
use ribbon_cache::{ Result, CACHE };
use ribbon_util::{ DISCORD_APP_ID, DISCORD_CLIENT };
use serde::{ Deserialize, Serialize };
use std::fmt::{ Display, Formatter };
use twilight_http::{
	request::{ Method, RequestBuilder },
	routing::Path
};
use twilight_model::{
	channel::message::EmojiReactionType,
	guild::Emoji as TwilightEmoji,
	id::{
		marker::EmojiMarker,
		Id
	}
};

static REQUEST_EMOJIS_PATH: Lazy<String> = Lazy::new(|| format!("applications/{}/emojis", *DISCORD_APP_ID));

#[derive(Deserialize)]
struct Emojis {
	items: Vec<TwilightEmoji>
}

#[derive(Serialize)]
struct CreateEmoji {
	image: String,
	name: String
}

pub async fn load_all() -> Result<()> {
	let request = RequestBuilder::raw(Method::Get, Path::ApplicationsMe, REQUEST_EMOJIS_PATH.clone())
		.build()?;
	let emojis = DISCORD_CLIENT
		.request::<Emojis>(request)
		.await?
		.model()
		.await?
		.items;
	let mut existing = Vec::new();
	for emoji in emojis {
		existing.push(emoji.name.clone());

		CACHE.discord.application_emojis.insert(emoji.id);
		CACHE.discord.emojis_mapped.insert(emoji.name.clone(), emoji.id);
		CACHE.discord.emojis.insert(emoji.id, emoji.into());
	}

	for item in Emoji::ITEMS {
		let name = item.name();
		if !existing.contains(&name) {
			info!("uploading emoji: {name}");

			let data = item.file_data();
			let data_encoded = BASE64_STANDARD.encode(data);

			let request = RequestBuilder::raw(Method::Post, Path::ApplicationsMe, REQUEST_EMOJIS_PATH.clone())
				.json(&CreateEmoji {
					image: format!("data:image/png;base64,{data_encoded}"),
					name
				})
				.build()?;
			let new_emoji = DISCORD_CLIENT
				.request::<TwilightEmoji>(request)
				.await?
				.model()
				.await?;

			CACHE.discord.application_emojis.insert(new_emoji.id);
			CACHE.discord.emojis_mapped.insert(new_emoji.name.clone(), new_emoji.id);
			CACHE.discord.emojis.insert(new_emoji.id, new_emoji.into());
		}
	}

	Ok(())
}

ribbon_emojis_macros::include_emojis!();

impl Display for Emoji {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		if let Some(emoji_id) = self.id() {
			if let Some(emoji) = CACHE.discord.emoji(emoji_id) {
				write!(f, "<:{}", emoji.name)?;
			} else {
				write!(f, "<:unknown")?;
			}
			write!(f, ":{emoji_id}>")
		} else { write!(f, "❓") }
	}
}

impl From<Emoji> for EmojiReactionType {
	fn from(value: Emoji) -> Self {
		if let Some(id) = value.id() {
			Self::Custom {
				animated: false,
				id,
				name: None
			}
		} else {
			Self::Unicode { name: "❓".to_string() }
		}
	}
}