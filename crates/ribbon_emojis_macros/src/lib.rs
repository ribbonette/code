use proc_macro::TokenStream;
use proc_macro2::{ Ident, Span };
use quote::quote;
use std::path::PathBuf;

fn str_to_pascal_case(string: &str) -> String {
	let mut pascal = String::new();
	let mut capitalize = true;
	for character in string.chars() {
		if character == '_' {
			capitalize = true;
		} else if capitalize {
			pascal.push(character.to_ascii_uppercase());
			capitalize = false;
		} else {
			pascal.push(character);
		}
	}

	pascal
}

fn workspace_dir() -> PathBuf {
	let manifest_path_str = std::env::var("CARGO_MANIFEST_DIR")
		.unwrap();
	let is_inner_crate = manifest_path_str.contains("crates");

	let mut manifest_path = PathBuf::from(manifest_path_str);
	if is_inner_crate {
		manifest_path.pop();
		manifest_path.pop();
	}
	
	manifest_path
}

#[proc_macro]
pub fn include_emojis(_item: TokenStream) -> TokenStream {
	let workspace_path = workspace_dir();
	let asset_entries: Vec<_> = std::fs::read_dir(workspace_path.join("assets/emojis"))
		.unwrap()
		.flatten()
		.collect();
	let entry_count = asset_entries.len();

	let mut names = Vec::with_capacity(entry_count);
	let mut items = Vec::with_capacity(entry_count);
	let mut variants = Vec::with_capacity(entry_count);
	for entry in asset_entries {
		let name = entry
			.file_name()
			.into_string()
			.unwrap()
			.replace(".png", "");
		let name_pascal = str_to_pascal_case(&name);
		let data = std::fs::read(entry.path())
			.unwrap();
		names.push(name.to_string());
		items.push(quote! { &[ #(#data),* ] });
		variants.push(Ident::new(&name_pascal, Span::call_site()));
	}

	TokenStream::from(quote! {
		use std::str::FromStr;

		pub enum Emoji {
			#(#variants),*
		}

		impl Emoji {
			pub const ITEMS: &'static [Emoji] = &[ #(Self::#variants),* ];

			pub fn id(&self) -> Option<Id<EmojiMarker>> {
				CACHE
					.discord
					.emoji_mapped(&self.name())
			}

			pub fn name(&self) -> String {
				match self {
					#( Self::#variants => #names.to_string() ),*
				}
			}

			pub fn file_data(&self) -> &[u8] {
				match self {
					#( Self::#variants => #items ),*
				}
			}
		}
	})
}