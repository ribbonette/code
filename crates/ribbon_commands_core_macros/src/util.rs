use syn::{ GenericArgument, PathArguments, Type };

pub fn extract_type_parameter<'a>(outer_type: &str, kind: &'a Type) -> Option<&'a Type> {
	if let Type::Path(path) = kind {
		if path.path.segments.len() == 1 {
			let path = &path.path.segments[0];
			if path.ident == outer_type {
				if let PathArguments::AngleBracketed(generics) = &path.arguments {
					if generics.args.len() == 1 {
						if let GenericArgument::Type(t) = &generics.args[0] {
							return Some(t);
						}
					}
				}
			}
		}
	}
	None
}