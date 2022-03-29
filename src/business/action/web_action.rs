use reqwest::StatusCode;

use crate::business::{
	action_type::automatic_action_type::AutomaticActionType,
	data::{
		action_data::{DescriptiveErrorInput, ErrorContext, ErrorData, ErrorInput, RequestInput},
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
	pub status: Option<u16>,
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
	AutomaticWebError(AutomaticErrorInput<(), WebInternalError>),
}

impl ActionError<AutomaticActionType, AutomaticRequestContext> for WebError {
	fn error_input(&self) -> DescriptiveErrorInput<AutomaticActionType, AutomaticRequestContext> {
		match &self {
			WebError::AutomaticError(error) => error.error_input(),
			WebError::AutomaticWebError(error) => error.to_descriptive(),
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
		AutomaticActionType::Web
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
/////////////////// INTERNAL ///////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct UrlData {
	url: String,
	status: Option<StatusCode>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ReqwestError {
	data: UrlData,
	error: reqwest::Error,
}

#[derive(Debug)]
pub enum WebInternalError {
	Reqwest(ReqwestError),
}

trait InternalErrorTrait<T> {
	fn to_error(self, url: T) -> WebInternalError;
}

impl InternalErrorTrait<String> for reqwest::Error {
	fn to_error(self, url: String) -> WebInternalError {
		WebInternalError::Reqwest(ReqwestError {
			data: UrlData {
				url,
				status: self.status(),
			},
			error: self,
		})
	}
}

////////////////////////////////////////////////
////////////////// FUNCTIONS ///////////////////
////////////////////////////////////////////////

fn run(data: &WebData) -> Result<WebResult, WebInternalError> {
	let url = format!(
		"http://httpbin.org{suffix}",
		suffix = if data.error {
			"/get/error".to_string()
		} else {
			if let Some(status) = data.status {
				format!("/status/{status}")
			} else {
				"/get".to_string()
			}
		}
	);
	reqwest::blocking::get(url.to_string())
		.map_err(|error| error.to_error(url.to_string()))?
		.error_for_status()
		.map_err(|error| error.to_error(url.to_string()))?
		.json::<WebResult>()
		.map_err(|error| error.to_error(url.to_string()))
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
			definition::action::{Action, ActionError, AutomaticAction},
		},
		tests::test_utils::tests::run_test,
	};

	#[test]
	fn test_internal_ok() {
		run_test(|_| {
			let context = automatic_context(AutomaticTestOptions { internal: true });

			let result = WebActionAutomatic::run(RequestInput {
				data: WebData {
					error: false,
					status: None,
				},
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
	fn test_decode_error() {
		run_test(|_| {
			let context = automatic_context(AutomaticTestOptions { internal: true });

			let result = WebActionAutomatic::run(RequestInput {
				data: WebData {
					error: true,
					status: None,
				},
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

			let description = result.unwrap_err().description();
			error!("description1={description}");
		});
	}

	#[test]
	fn test_status_error() {
		run_test(|_| {
			let context = automatic_context(AutomaticTestOptions { internal: true });

			let result = WebActionAutomatic::run(RequestInput {
				data: WebData {
					error: false,
					status: Some(403),
				},
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

			let description = result.unwrap_err().description();
			error!("description2={description}");
		});
	}
}
