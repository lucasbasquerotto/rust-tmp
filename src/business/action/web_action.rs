use crate::business::{
	action_type::automatic_action_type::AutomaticActionType,
	data::{
		action_data::{DescriptiveError, ErrorData, ErrorInfo, RequestInput},
		automatic_action_data::{AutomaticActionError, AutomaticRequestContext},
	},
	definition::action::{ActionError, ActionInput, ActionOutput, AutomaticAction},
};

#[cfg(not(test))]
fn httpbin_base_url() -> String {
	"http://httpbin.org".to_string()
}

#[cfg(test)]
fn httpbin_base_url() -> String {
	format!(
		"{host}/{path}",
		host = &mockito::SERVER_URL,
		path = "/mock/http"
	)
}

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
	WebError(WebSharedError),
}

impl ActionError for WebError {
	fn private_error(&self) -> DescriptiveError {
		match &self {
			WebError::AutomaticError(error) => error.private_error(),
			WebError::WebError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match &self {
			WebError::AutomaticError(error) => error.public_error(),
			WebError::WebError(error) => error.public_error(),
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
		run(&input.data).map_err(|err| WebError::WebError(err))
	}
}

////////////////////////////////////////////////
/////////////////// INTERNAL ///////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq)]
pub struct UrlData {
	url: String,
	status: Option<u16>,
}

#[derive(Debug, PartialEq)]
pub enum WebSharedError {
	Reqwest(ErrorInfo<UrlData, reqwest::Error>),
}

impl ActionError for WebSharedError {
	fn private_error(&self) -> DescriptiveError {
		match &self {
			WebSharedError::Reqwest(source) => DescriptiveError::source(source),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		let msg = match &self {
			WebSharedError::Reqwest(info) => {
				#[cfg(not(test))]
				let url_data = &info.data;

				#[cfg(test)]
				let url_data = &info.data;

				match url_data.status {
					Some(status_code) => match &status_code {
						403 => "Web Action - Forbidden".to_string(),
						404 => "Web Action - Not Found".to_string(),
						status => format!("Web error -> Status: {status}"),
					},
					None => "Web error occured".to_string(),
				}
			}
		};
		Self::error_msg(msg)
	}
}

trait SharedErrorTrait<T> {
	fn to_error(self, url: T) -> WebSharedError;
}

impl SharedErrorTrait<String> for reqwest::Error {
	fn to_error(self, url: String) -> WebSharedError {
		let data = UrlData {
			url,
			status: self.status().map(|status| status.as_u16()),
		};

		#[cfg(not(test))]
		let input = ErrorInfo { data, source: self };

		#[cfg(test)]
		let input = ErrorInfo {
			data,
			source: Some(self),
		};

		WebSharedError::Reqwest(input)
	}
}

////////////////////////////////////////////////
////////////////// FUNCTIONS ///////////////////
////////////////////////////////////////////////

fn run(data: &WebData) -> Result<WebResult, WebSharedError> {
	let url = format!(
		"{host}{suffix}",
		host = httpbin_base_url(),
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
			action::web_action::{
				httpbin_base_url, UrlData, WebActionAutomatic, WebData, WebError, WebResult,
				WebResultArgs, WebSharedError,
			},
			data::{
				action_data::{ErrorInfo, RequestInput},
				automatic_action_data::tests::{automatic_context, AutomaticTestOptions},
			},
			definition::action::{Action, ActionError},
		},
		tests::test_utils::tests::run_test,
	};

	#[test]
	fn test_internal_ok() {
		run_test(|_| {
			let context = automatic_context(AutomaticTestOptions { internal: true });

			let _m = mock("GET", "/mock/http/get")
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
	fn test_status_error() {
		run_test(|_| {
			let context = automatic_context(AutomaticTestOptions { internal: true });

			let _m = mock("GET", "/mock/http/status/403")
				.with_status(403)
				.create();

			let result = WebActionAutomatic::run(RequestInput {
				data: WebData {
					error: false,
					status: Some(403),
				},
				context: context.clone(),
			});

			assert_eq!(
				&result,
				&Err(WebError::WebError(WebSharedError::Reqwest(
					ErrorInfo::mock(UrlData {
						url: format!(
							"{host}/{path}",
							host = httpbin_base_url(),
							path = "/status/403"
						),
						status: Some(403)
					})
				)))
			);

			let public_error = &result.unwrap_err().public_error();

			assert_eq!(
				public_error.as_ref().unwrap().msg,
				"Web Action - Forbidden".to_string()
			);
		});
	}

	#[test]
	fn test_no_status_error() {
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
				&Err(WebError::WebError(WebSharedError::Reqwest(
					ErrorInfo::mock(UrlData {
						url: format!(
							"{host}/{path}",
							host = httpbin_base_url(),
							path = "/get/error"
						),
						status: None
					})
				)))
			);

			let public_error = &result.unwrap_err().public_error();

			assert_eq!(
				public_error.as_ref().unwrap().msg,
				"Web error occured".to_string()
			);
		});
	}
}
