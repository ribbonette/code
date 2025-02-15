use futures::future::BoxFuture;
use serde::Serialize;
use serde_repr::Serialize_repr;
use twilight_model::{
	application::command::CommandOptionChoice,
	channel::ChannelType,
	guild::Permissions
};

use crate::{
	util::serialize_option_as_bool,
	Context, CoreError
};

pub struct Command {
	pub contexts: Vec<CommandContext>,
	pub default_member_permissions: Option<u64>,
	pub description: Option<String>,
	pub handler: fn(Context) -> BoxFuture<'static, Result<(), CoreError>>,
	// TODO: consolidate these is_.. things
	pub is_message: bool,
	pub is_slash: bool,
	pub is_user: bool,
	pub name: String,
	pub options: Vec<CommandOption>,
	pub subcommands: Vec<Command>
}

impl Command {
	pub fn default_member_permissions(&self) -> Option<Permissions> {
		self.default_member_permissions.map(Permissions::from_bits_truncate)
	}
}

#[derive(Clone, Serialize_repr)]
#[repr(u8)]
pub enum CommandContext {
	Guild,
	BotDM,
	PrivateChannel
}

#[derive(Clone, Serialize)]
pub struct CommandOption {
	#[serde(serialize_with = "serialize_option_as_bool")]
	#[allow(clippy::type_complexity)]
	pub autocomplete: Option<fn(Context, String) -> BoxFuture<'static, Result<Vec<CommandOptionChoice>, CoreError>>>,

	#[serde(rename = "channel_types")]
	pub channel_kinds: Option<Vec<ChannelType>>,
	pub description: Option<String>,
	#[serde(rename = "type")]
	pub kind: CommandOptionKind,
	pub name: String,
	pub options: Vec<CommandOption>,
	
	pub required: bool
}

#[derive(Clone, Serialize_repr)]
#[repr(u8)]
pub enum CommandOptionKind {
	SubCommand = 1,
	SubCommandGroup,
	String,
	Integer,
	Boolean,
	User,
	Channel,
	Role,
	Mentionable,
	Number,
	Attachment
}