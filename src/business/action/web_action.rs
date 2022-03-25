use crate::business::{
	action_type::automatic_action_type::AutomaticActionType,
	data::{
		action_data::{ErrorContext, ErrorData, ErrorInput, RequestInput},
		automatic_action_data::{
			AutomaticActionError, AutomaticErrorInput, AutomaticRequestContext,
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
pub struct WebResultArgs {}

#[derive(Debug, PartialEq, Deserialize)]
pub struct WebResult {
	pub url: String,
	pub args: WebResultArgs,
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
pub struct WebActionAutomatic(RequestInput<WebData, AutomaticRequestContext>);

impl AutomaticAction<WebData, WebResult, WebError> for WebActionAutomatic {
	fn action_type() -> AutomaticActionType {
		AutomaticActionType::Auto
	}

	fn new(
		input: Result<RequestInput<WebData, AutomaticRequestContext>, AutomaticActionError>,
	) -> Result<Self, WebError> {
		input
			.map(|ok_input| Self(ok_input))
			.map_err(|err| WebError::AutomaticError(err))
	}

	fn run_inner(self) -> Result<WebResult, WebError> {
		let WebActionAutomatic(input) = &self;
		run(&input.data).map_err(|err| {
			WebError::AutomaticWebError(ErrorInput {
				error_context: ErrorContext {
					action_type: Self::action_type(),
					context: input.context.clone(),
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

fn run(data: &WebData) -> Result<WebResult, reqwest::Error> {
	reqwest::blocking::get(format!(
		"http://httpbin.org/get{error}",
		error = if data.error { "/error" } else { "" }
	))?
	.json::<WebResult>()
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use crate::{
		business::{
			action::web_action::{WebActionAutomatic, WebData, WebError, WebResult, WebResultArgs},
			data::{
				action_data::{ErrorContext, ErrorInput, RequestInput},
				automatic_action_data::tests::{automatic_context, AutomaticTestOptions},
			},
			definition::action::{Action, AutomaticAction},
		},
		tests::test_utils::tests::run_test,
	};

	#[test]
	fn test_internal_ok() {
		run_test(|_| {
			let context = automatic_context(AutomaticTestOptions { internal: true });

			let result = WebActionAutomatic::run(RequestInput {
				data: WebData { error: false },
				context: context.clone(),
			});

			assert_eq!(
				&result,
				&Ok(WebResult {
					url: "http://httpbin.org/get".to_string(),
					args: WebResultArgs {}
				})
			);
		});
	}

	#[test]
	fn test_internal_error() {
		run_test(|_| {
			let context = automatic_context(AutomaticTestOptions { internal: true });

			let result = WebActionAutomatic::run(RequestInput {
				data: WebData { error: true },
				context: context.clone(),
			});

			assert_eq!(
				&result,
				&Err(WebError::AutomaticWebError(ErrorInput {
					error_context: ErrorContext {
						action_type: WebActionAutomatic::action_type(),
						context: context.clone()
					},
					data: (),
					source: None
				}))
			);
		});
	}
}
