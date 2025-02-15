use twilight_model::{
	guild::Emoji,
	id::{
		marker::EmojiMarker,
		Id
	}
};

pub struct EmojiModel {
	pub id: Id<EmojiMarker>,
	pub name: String
}

impl From<Emoji> for EmojiModel {
	fn from(value: Emoji) -> Self {
		Self {
			id: value.id,
			name: value.name
		}
	}
}