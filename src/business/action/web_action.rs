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
	AutomaticWebError(AutomaticErrorInput<(), WebSharedError>),
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
			WebError::AutomaticWebError(input) => self.error_msg(match &input.source {
				Some(source) => match &source {
					WebSharedError::Reqwest(info) => match &info.data.status {
						Some(status_code) => match &status_code.as_u16() {
							403 => "Web Action - Forbidden".to_string(),
							404 => "Web Action - Not Found".to_string(),
							status => format!("Web error -> Status: {status}"),
						},
						None => "Web error occured".to_string(),
					},
				},
				None => "Unknown web error occured".to_string(),
			}),
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
pub enum WebSharedError {
	Reqwest(ReqwestError),
}

trait SharedErrorTrait<T> {
	fn to_error(self, url: T) -> WebSharedError;
}

impl SharedErrorTrait<String> for reqwest::Error {
	fn to_error(self, url: String) -> WebSharedError {
		WebSharedError::Reqwest(ReqwestError {
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

fn run(data: &WebData) -> Result<WebResult, WebSharedError> {
	#[cfg(not(test))]
	let host = "http://httpbin.org";

	// The host to be used in test compilation
	#[cfg(test)]
	let host = &mockito::SERVER_URL;

	let url = format!(
		"{host}{suffix}",
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

	if data.error {
		reqwest::blocking::get(url.to_string())
			.map_err(|error| error.to_error(url.to_string()))?
			.json::<WebResult>()
			.map_err(|error| error.to_error(url.to_string()))
	} else {
		reqwest::blocking::get(url.to_string())
			.map_err(|error| error.to_error(url.to_string()))?
			.error_for_status()
			.map_err(|error| error.to_error(url.to_string()))?
			.json::<WebResult>()
			.map_err(|error| error.to_error(url.to_string()))
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use mockito::mock;

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

			let _m = mock("GET", "/get")
				.with_status(200)
				.with_body(
					r##"
					{
						"args": {},
						"origin": "localhost",
						"url": "http://httpbin.org.mock/get"
					}
					"##,
				)
				.create();

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
					url: "http://httpbin.org.mock/get".to_string(),
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

			let public_error = &result.unwrap_err().public_error();

			assert_eq!(
				public_error.as_ref().unwrap().msg,
				"Web error occured".to_string()
			);
		});
	}

	#[test]
	fn test_status_error() {
		run_test(|_| {
			let context = automatic_context(AutomaticTestOptions { internal: true });

			let _m = mock("GET", "/status/403").with_status(403).create();

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

			let public_error = &result.unwrap_err().public_error();

			assert_eq!(
				public_error.as_ref().unwrap().msg,
				"Web Action - Forbidden".to_string()
			);
		});
	}
}
