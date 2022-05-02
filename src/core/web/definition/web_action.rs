use crate::{
	core::action::{
		action_type::general_action_type::ActionType,
		data::action_data::{ErrorData, RequestInput},
		definition::{
			action::{ActionError, ActionInput, ActionOutput},
			action_helpers::DescriptiveRequestContext,
		},
	},
	lib::traits::async_from::AsyncInto,
};
use rocket::serde::json::Json;
use std::fmt::Debug;

pub type WebActionResult<O> = Result<Json<O>, Json<Option<ErrorData>>>;

#[rocket::async_trait]
pub trait WebAction<I, O, E, R, C, A, N>: Debug
where
	I: ActionInput + Send,
	O: ActionOutput,
	E: ActionError,
	R: Into<E> + Send,
	C: DescriptiveRequestContext + Send,
	A: ActionType,
	N: AsyncInto<Result<RequestInput<I, C>, R>> + Send + 'static,
{
	async fn request(input: N) -> Result<Json<O>, Json<Option<ErrorData>>>;
}
