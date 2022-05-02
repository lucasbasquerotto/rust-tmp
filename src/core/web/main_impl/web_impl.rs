use crate::{
	core::{
		action::{
			action_type::general_action_type::ActionType,
			data::action_data::{ActionErrorInfo, ActionResultInfo, ErrorData, RequestInput},
			definition::{
				action::{Action, ActionError, ActionInput, ActionOutput},
				action_helpers::{ActionErrorHelper, DescriptiveRequestContext},
			},
		},
		web::definition::web_action::WebAction,
	},
	lib::{data::result::AsyncResult, traits::async_from::AsyncInto},
};
use rocket::serde::json::Json;

impl<I, O, E, R, C, A, T, N> WebAction<I, O, E, R, C, A, N> for T
where
	I: ActionInput + Send,
	O: ActionOutput,
	E: ActionError,
	R: Into<E> + Send,
	C: DescriptiveRequestContext + Send,
	A: ActionType,
	N: AsyncInto<Result<RequestInput<I, C>, R>> + Send + 'static,
	T: Action<Result<RequestInput<I, C>, R>, ActionResultInfo<A, C, O>, ActionErrorInfo<A, C, E>>,
{
	fn request(input: N) -> AsyncResult<Json<O>, Json<Option<ErrorData>>> {
		Box::pin(async {
			let res = Self::run(input.into().await)
				.await
				.map(|out| Json(out.data))
				.map_err(|err| {
					let err = err.handle();
					Json(err)
				});
			res
		})
	}
}
