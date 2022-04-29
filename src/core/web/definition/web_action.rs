use crate::{
	core::action::{
		action_type::general_action_type::ActionType,
		data::action_data::{ErrorData, RequestInput},
		definition::{
			action::{ActionError, ActionInput, ActionOutput},
			action_helpers::DescriptiveRequestContext,
		},
	},
	lib::data::result::AsyncResult,
};
use rocket::serde::json::Json;
use std::fmt::Debug;

pub type WebActionResult<O> = Result<Json<O>, Json<Option<ErrorData>>>;

pub trait WebAction<I, O, E, R, C, A, N>: Debug
where
	I: ActionInput + Send,
	O: ActionOutput,
	E: ActionError,
	R: Into<E> + Send,
	C: DescriptiveRequestContext + Send,
	A: ActionType,
	N: Into<Result<RequestInput<I, C>, R>> + Send + 'static,
{
	fn request(input: N) -> AsyncResult<Json<O>, Json<Option<ErrorData>>>;
}
