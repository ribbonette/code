use darling::{ ast::NestedMeta, FromMeta, Result };

#[derive(Debug)]
pub struct List<T>(pub Vec<T>);

impl<T> Default for List<T> {
	fn default() -> Self {
		Self(Default::default())
	}
}

impl<T: FromMeta> FromMeta for List<T> {
	fn from_list(items: &[NestedMeta]) -> Result<Self> {
		items
			.iter()
			.map(T::from_nested_meta)
			.collect::<Result<Vec<T>>>()
			.map(Self)
	}
}