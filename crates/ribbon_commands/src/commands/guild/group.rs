use ribbon_commands_core::{ Context, Result, command };

#[command(slash, context = "guild", description = "group", subcommands("group_add"))]
pub async fn group(_context: Context) -> Result<()> {
	unreachable!()
}

#[command(slash, rename = "add", context = "guild", description = "test")]
pub async fn group_add(context: Context) -> Result<()> {
	context
		.reply("Before you can use this feature, you'll need to grant me additional access on Roblox.")
		.await
}