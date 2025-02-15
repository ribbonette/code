use serde::Serializer;

pub fn serialize_option_as_bool<T, S: Serializer>(value: &Option<T>, serialiser: S) -> Result<S::Ok, S::Error> {
	serialiser.serialize_bool(value.is_some())
}