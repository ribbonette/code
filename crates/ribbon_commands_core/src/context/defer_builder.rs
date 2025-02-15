use twilight_model::{
	channel::message::MessageFlags,
	http::interaction::{ InteractionResponse, InteractionResponseType }
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::Result;
use super::Context;

pub struct DeferBuilder<'a> {
	context: &'a Context,
	data: InteractionResponseDataBuilder
}

impl<'a> DeferBuilder<'a> {
	pub fn new(context: &'a Context) -> Self {
		Self {
			context,
			data: InteractionResponseDataBuilder::new()
		}
	}

	pub fn ephemeral(mut self) -> Self {
		self.data = self.data.flags(MessageFlags::EPHEMERAL);
		self
	}
}

impl IntoFuture for DeferBuilder<'_> {
	type Output = Result<()>;
	type IntoFuture = impl Future<Output = Result<()>>;

	fn into_future(self) -> Self::IntoFuture {
		self.context.response(InteractionResponse {
			kind: InteractionResponseType::DeferredChannelMessageWithSource,
			data: Some(self.data.build())
		})
	}
}