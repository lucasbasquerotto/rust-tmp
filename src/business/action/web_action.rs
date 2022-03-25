use crate::business::{
	action_type::automatic_action_type::AutomaticActionType,
	data::{
		action_data::{ErrorContext, ErrorData, ErrorInput, RequestInput},
		automatic_action_data::{
			AutomaticActionError, AutomaticErrorInput, AutomaticRequestContext, HookRequestContext,
			InternalRequestContext,
		},
	},
	definition::{
		action::{ActionError, ActionInput, ActionOutput, AutomaticAction},
		action_helpers::ActionErrorHelper,
	},
};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct WebData {
	pub error: bool,
}

impl ActionInput for WebData {}

////////////////////////////////////////////////
//////////////////// OUTPUT ////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq, Deserialize)]
pub struct WebResult {
	pub url: String,
}

impl ActionOutput for WebResult {}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum WebError {
	AutomaticError(AutomaticActionError),
	AutomaticWebError(AutomaticErrorInput<(), reqwest::Error>),
}

impl ActionError<AutomaticActionType, AutomaticRequestContext> for WebError {
	fn error_context(&self) -> &ErrorContext<AutomaticActionType, AutomaticRequestContext> {
		match &self {
			WebError::AutomaticError(error) => error.error_context(),
			WebError::AutomaticWebError(error) => &error.error_context,
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match &self {
			WebError::AutomaticError(error) => error.public_error(),
			WebError::AutomaticWebError(_) => self.error_msg(format!("Web error occured")),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct WebActionInternal(RequestInput<WebData, InternalRequestContext>);

impl AutomaticAction<WebData, WebResult, WebError> for WebActionInternal {
	fn action_type() -> AutomaticActionType {
		AutomaticActionType::Auto
	}

	fn new(
		input: Result<RequestInput<WebData, AutomaticRequestContext>, AutomaticActionError>,
	) -> Result<Self, WebError> {
		input
			.and_then(|ok_input| ok_input.to_internal(Self::action_type()))
			.map(|ok_input| Self(ok_input))
			.map_err(|err| WebError::AutomaticError(err))
	}

	fn run_inner(self) -> Result<WebResult, WebError> {
		let WebActionInternal(input) = &self;
		run().map_err(|err| {
			WebError::AutomaticWebError(ErrorInput {
				error_context: ErrorContext {
					action_type: Self::action_type(),
					context: input.context.to_general(),
				},
				data: (),
				source: Some(err),
			})
		})
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct AutoActionHook(RequestInput<WebData, HookRequestContext>);

impl AutomaticAction<WebData, WebResult, WebError> for AutoActionHook {
	fn action_type() -> AutomaticActionType {
		AutomaticActionType::Auto
	}

	fn new(
		input: Result<RequestInput<WebData, AutomaticRequestContext>, AutomaticActionError>,
	) -> Result<Self, WebError> {
		match input {
			Err(err) => Err(WebError::AutomaticError(err)),
			Ok(ok_input) => {
				let real_input = ok_input.to_hook(Self::action_type());

				match real_input {
					Err(err) => Err(WebError::AutomaticError(err)),
					Ok(real_ok_input) => Ok(Self(real_ok_input)),
				}
			}
		}
	}

	fn run_inner(self) -> Result<WebResult, WebError> {
		let AutoActionHook(input) = &self;
		run().map_err(|err| {
			WebError::AutomaticWebError(ErrorInput {
				error_context: ErrorContext {
					action_type: Self::action_type(),
					context: input.context.to_general(),
				},
				data: (),
				source: Some(err),
			})
		})
	}
}

////////////////////////////////////////////////
////////////////// FUNCTIONS ///////////////////
////////////////////////////////////////////////

fn run() -> Result<WebResult, reqwest::Error> {
	reqwest::blocking::get("http://httpbin.org/get")?.json::<WebResult>()
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use crate::{
		business::{
			action::web_action::{WebActionInternal, WebData, WebResult},
			data::{
				action_data::RequestInput,
				automatic_action_data::tests::{automatic_context, AutomaticTestOptions},
			},
			definition::action::Action,
		},
		tests::test_utils::tests::run_test,
	};

	#[test]
	fn test_internal_ok() {
		run_test(|_| {
			let context = automatic_context(AutomaticTestOptions { internal: true });

			let result = WebActionInternal::run(RequestInput {
				data: WebData { error: false },
				context: context.clone(),
			});

			assert_eq!(
				&result,
				&Ok(WebResult {
					url: "http://httpbin.org/get".to_string(),
				})
			);
		});
	}
}
