use ribbon_commands_core::command::{ Command, CommandContext, CommandOption, CommandOptionKind };
use serde::Serialize;
use twilight_model::{
	guild::Permissions,
	application::command::CommandType
};

use crate::Result;

#[derive(Serialize)]
pub struct ApplicationCommand {
	default_member_permissions: Option<Permissions>,
	description: String,
	contexts: Vec<CommandContext>,

	#[serde(rename = "type")]
	kind: CommandType,
	name: String,
	options: Vec<CommandOption>,
}

impl ApplicationCommand {
	pub fn new(command: &Command, kind: CommandType) -> Result<Self> {
		let description = match kind {
			CommandType::User => "",
			_ => command.description.as_ref().map_or("there is no description yet, how sad...", |x| x.as_str())
		};
		let mut options = command.options.clone();
		for subcommand in command.subcommands.iter() {
			options.push(CommandOption {
				kind: CommandOptionKind::SubCommand,
				name: subcommand.name.clone(),
				required: false,
				description: subcommand.description.clone().or(Some("there is no description yet, how sad...".into())),
				autocomplete: None,
				channel_kinds: None,
				options: subcommand.options.clone()
			});
		}
	
		Ok(Self {
			kind,
			name: command.name.clone(),
			options,
			contexts: command.contexts.clone(),
			description: description.to_string(),
			default_member_permissions: command.default_member_permissions()
		})
	}
}