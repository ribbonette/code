use darling::{ ast::NestedMeta, FromMeta };
use proc_macro::TokenStream;
use quote::{ ToTokens, format_ident, quote };
use syn::{ spanned::Spanned, Expr, FnArg, ItemFn, Pat, Path, parse_quote };

use list::List;

mod list;
mod util;

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
struct CommandArgs {
	#[darling(multiple, rename = "context")]
	contexts: Vec<String>,
	default_member_permissions: Option<u64>,
	description: Option<String>,
	message: bool,
	rename: Option<String>,
	subcommands: List<Path>,
	slash: bool,
	user: bool,
}

#[derive(Default, Debug, FromMeta)]
#[darling(default)]
struct ParamArgs {
	autocomplete: Option<Path>,
	channel_kinds: Option<List<String>>,
	description: String,
	rename: Option<String>
}

struct CommandOption {
	name: String,
	kind: syn::Type,
	blah: proc_macro2::TokenStream
}

fn wrap_option_to_string<T: ToTokens>(literal: Option<T>) -> Expr {
	match literal {
		Some(literal) => parse_quote! { Some(#literal.to_string()) },
		None => parse_quote! { None },
	}
}

fn wrap_option<T: ToTokens>(literal: Option<T>) -> Expr {
	match literal {
		Some(literal) => parse_quote! { Some(#literal) },
		None => parse_quote! { None },
	}
}

fn create_command(args: TokenStream, mut function: ItemFn) -> Result<TokenStream, darling::Error> {
	let args = NestedMeta::parse_meta_list(args.into())?;
	let args = <CommandArgs as FromMeta>::from_list(&args)?;
	if !args.user && !args.slash && !args.message {
		return Err(syn::Error::new(function.sig.span(), "command must specify either user, slash, or message").into());
	}

	let function_name = function
		.sig
		.ident
		.to_string()
		.trim_start_matches("r#")
		.to_string();

	let function_ident = std::mem::replace(&mut function.sig.ident, parse_quote! { inner });
	let function_generics = &function.sig.generics;
	let function_visibility = &function.vis;

	let contexts: Vec<Expr> = args.contexts.into_iter().map(|x| match x.as_str() {
		"guild" => parse_quote! { ribbon_commands_core::command::CommandContext::Guild },
		"bot_dm" => parse_quote! { ribbon_commands_core::command::CommandContext::BotDM },
		"private_channel" => parse_quote! { ribbon_commands_core::command::CommandContext::PrivateChannel },
		_ => panic!("invalid context, must specify either guild, bot_dm, or private_channel")
	}).collect();
	let is_user = args.user;
	let is_slash = args.slash;
	let is_message = args.message;
	let description = wrap_option_to_string(args.description);
	let default_member_permissions = wrap_option(args.default_member_permissions);

	let mut parameters: Vec<CommandOption> = vec![];
	for command_param in function.sig.inputs.iter_mut().skip(1) {
		let pattern = match command_param {
			FnArg::Typed(x) => x,
			FnArg::Receiver(r) => {
				return Err(syn::Error::new(r.span(), "self argument is invalid here").into());
			}
		};

		let attrs: Vec<_> = pattern
			.attrs
			.drain(..)
			.map(|attr| NestedMeta::Meta(attr.meta))
			.collect();
		let attrs = <ParamArgs as FromMeta>::from_list(&attrs)?;

		let name = if let Some(rename) = &attrs.rename {
			rename.clone()
		} else if let Pat::Ident(ident) = &*pattern.pat {
			ident.ident.to_string().trim_start_matches("r#").into()
		} else {
			let message = "#[rename = \"...\"] must be specified for pattern parameters";
			return Err(syn::Error::new(pattern.pat.span(), message).into());
		};
		let description = attrs.description;

		let autocomplete = match attrs.autocomplete {
			Some(autocomplete_fn) => quote! {
				Some(|interaction, partial| Box::pin(async move {
					#autocomplete_fn(interaction, partial)
						.await
						.map_err(|error| ribbon_commands_core::CoreError {
							kind: ribbon_commands_core::CoreErrorKind::Autocomplete,
							source: error.into()
						})
				}))
			},
			None => quote! { None }
		};

		let channel_kinds = match attrs.channel_kinds {
			Some(List(channel_kinds)) => {
				let kinds = channel_kinds
					.into_iter()
					.map(|x| match x.as_str() {
						"guild_text" => parse_quote! { twilight_model::channel::ChannelType::GuildText },
						"private" => parse_quote! { twilight_model::channel::ChannelType::Private },
						"guild_voice" => parse_quote! { twilight_model::channel::ChannelType::GuildVoice },
						"group" => parse_quote! { twilight_model::channel::ChannelType::Group },
						"guild_category" => parse_quote! { twilight_model::channel::ChannelType::GuildCategory },
						"guild_announcement" => parse_quote! { twilight_model::channel::ChannelType::GuildAnnouncement },
						"announcement_thread" => parse_quote! { twilight_model::channel::ChannelType::AnnouncementThread },
						"public_thread" => parse_quote! { twilight_model::channel::ChannelType::PublicThread },
						"private_thread" => parse_quote! { twilight_model::channel::ChannelType::PrivateThread },
						"guild_stage_voice" => parse_quote! { twilight_model::channel::ChannelType::GuildStageVoice },
						"guild_directory" => parse_quote! { twilight_model::channel::ChannelType::GuildDirectory },
						"guild_forum" => parse_quote! { twilight_model::channel::ChannelType::GuildForum },
						"guild_media" => parse_quote! { twilight_model::channel::ChannelType::GuildMedia },
						_ => panic!("invalid context, must specify either guild, bot_dm, or private_channel")
					})
					.collect::<Vec<Expr>>();
				quote! { Some(vec![ #( #kinds),* ]) }
			},
			None => quote! { None }
		};

		let kind = &pattern.ty;
		let extracted_type = util::extract_type_parameter("Option", kind);
		let required = extracted_type.is_none();
		let final_type = extracted_type.unwrap_or(kind);
		let option_kind = quote! {
			ribbon_commands_core::create_slash_argument!(#final_type)
		};
		parameters.push(CommandOption {
			name: name.clone(),
			kind: *kind.clone(),
			blah: quote! {
				ribbon_commands_core::command::CommandOption {
					name: #name.to_string(),
					kind: #option_kind,
					required: #required,
					description: Some(#description.to_string()),
					autocomplete: #autocomplete,
					channel_kinds: #channel_kinds,
					options: Vec::new()
				}
			}
		});
	}

	let param_identifiers = (0..parameters.len())
		.map(|i| format_ident!("ribbon_param_{i}"))
		.collect::<Vec<_>>();
	let param_names = parameters.iter().map(|p| &p.name).collect::<Vec<_>>();

	let param_types = parameters
		.iter()
		.map(|p| {
			let t = &p.kind;
			/*if p.args.flag {
				quote! { FLAG }
			} else if let Some(choices) = &p.args.choices {
				let choice_indices = (0..choices.0.len()).map(syn::Index::from);
				let choice_vals = &choices.0;
				quote! { INLINE_CHOICE #t [#(#choice_indices: #choice_vals),*] }
			} else {
				quote! { #t }
			}*/
			quote! { #t }
		})
		.collect::<Vec<_>>();

	let handler = quote! {
		|context| Box::pin(async move {
			let ( #( #param_identifiers, )* ) = ribbon_commands_core::parse_command_arguments!(
				&context, &context.options =>
				#( (#param_names: #param_types), )*
			)
				.await
				.map_err(|error| ribbon_commands_core::CoreError {
					kind: ribbon_commands_core::CoreErrorKind::CommandArguments,
					source: error.into()
				})?;

			inner(context, #( #param_identifiers, )*)
				.await
				.map_err(|error| ribbon_commands_core::CoreError {
					kind: ribbon_commands_core::CoreErrorKind::Command,
					source: error.into()
				})
		})
	};

	let name = args.rename.unwrap_or(function_name);
	let options: Vec<proc_macro2::TokenStream> = parameters.into_iter().map(|x| x.blah).collect();
	let subcommands = args.subcommands.0;
	Ok(TokenStream::from(quote::quote! {
		#function_visibility fn #function_ident #function_generics() -> ribbon_commands_core::Command {
			#function
			ribbon_commands_core::Command {
				name: #name.to_string(),
				options: vec![ #( #options ),* ],
				contexts: vec![ #( #contexts ),* ],
				handler: #handler,
				is_user: #is_user,
				is_slash: #is_slash,
				is_message: #is_message,
				description: #description,
				default_member_permissions: #default_member_permissions,
				subcommands: vec![ #( #subcommands() ),* ]
			}
		}
	}))
}

#[proc_macro_attribute]
pub fn command(args: TokenStream, function: TokenStream) -> TokenStream {
	let function = syn::parse_macro_input!(function as ItemFn);
	match create_command(args, function) {
		Ok(x) => x,
		Err(x) => x.write_errors().into()
	}
}