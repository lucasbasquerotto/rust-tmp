use crate::core::action::{
	action_type::{
		automatic_action_type::AutomaticActionType, moderator_action_type::ModeratorActionType,
		user_action_type::UserActionType,
	},
	data::{
		action_data::{DescriptiveError, ErrorData, ErrorInfo},
		automatic_action_data::{AutomaticActionError, AutomaticRequestInput},
		moderator_action_data::{
			ModeratorActionError, ModeratorActionInput, ModeratorRequestInput,
		},
		user_action_data::{UserActionError, UserActionInput, UserRequestInput},
	},
};
use crate::core::action::{
	data::automatic_action_data::AutomaticActionInput,
	definition::action::{
		ActionError, ActionInput, ActionOutput, AutomaticAction, ModeratorAction, UserAction,
	},
};

////////////////////////////////////////////////
//////////////////// STATIC ////////////////////
////////////////////////////////////////////////

#[cfg(not(test))]
fn httpbin_base_url() -> String {
	"http://httpbin.org".to_string()
}

#[cfg(test)]
fn httpbin_base_url() -> String {
	format!(
		"{host}/{path}",
		host = &mockito::SERVER_URL,
		path = "mock/http"
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
pub enum UserWebError {
	UserError(UserActionError),
	WebError(WebSharedError),
}

impl ActionError for UserWebError {
	fn private_error(&self) -> DescriptiveError {
		match &self {
			UserWebError::UserError(error) => error.private_error(),
			UserWebError::WebError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match &self {
			UserWebError::UserError(error) => error.public_error(),
			UserWebError::WebError(error) => error.public_error(),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct WebActionUser(UserRequestInput<WebData>);

impl UserAction<WebData, WebResult, UserWebError> for WebActionUser {
	fn action_type() -> UserActionType {
		UserActionType::Web
	}

	fn new(input: UserActionInput<WebData>) -> Result<Self, UserWebError> {
		input.map(Self).map_err(UserWebError::UserError)
	}

	fn run_inner(self) -> Result<WebResult, UserWebError> {
		let WebActionUser(input) = &self;
		run(&input.data).map_err(UserWebError::WebError)
	}
}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum ModeratorWebError {
	ModeratorError(ModeratorActionError),
	WebError(WebSharedError),
}

impl ActionError for ModeratorWebError {
	fn private_error(&self) -> DescriptiveError {
		match &self {
			ModeratorWebError::ModeratorError(error) => error.private_error(),
			ModeratorWebError::WebError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match &self {
			ModeratorWebError::ModeratorError(error) => error.public_error(),
			ModeratorWebError::WebError(error) => error.public_error(),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct WebActionModerator(ModeratorRequestInput<WebData>);

impl ModeratorAction<WebData, WebResult, ModeratorWebError> for WebActionModerator {
	fn action_type() -> ModeratorActionType {
		ModeratorActionType::Web
	}

	fn new(input: ModeratorActionInput<WebData>) -> Result<Self, ModeratorWebError> {
		input.map(Self).map_err(ModeratorWebError::ModeratorError)
	}

	fn run_inner(self) -> Result<WebResult, ModeratorWebError> {
		let WebActionModerator(input) = &self;
		run(&input.data).map_err(ModeratorWebError::WebError)
	}
}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum AutomaticWebError {
	AutomaticError(AutomaticActionError),
	WebError(WebSharedError),
}

impl ActionError for AutomaticWebError {
	fn private_error(&self) -> DescriptiveError {
		match &self {
			AutomaticWebError::AutomaticError(error) => error.private_error(),
			AutomaticWebError::WebError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match &self {
			AutomaticWebError::AutomaticError(error) => error.public_error(),
			AutomaticWebError::WebError(error) => error.public_error(),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct WebActionAutomatic(AutomaticRequestInput<WebData>);

impl AutomaticAction<WebData, WebResult, AutomaticWebError> for WebActionAutomatic {
	fn action_type() -> AutomaticActionType {
		AutomaticActionType::Web
	}

	fn new(input: AutomaticActionInput<WebData>) -> Result<Self, AutomaticWebError> {
		input.map(Self).map_err(AutomaticWebError::AutomaticError)
	}

	fn run_inner(self) -> Result<WebResult, AutomaticWebError> {
		let WebActionAutomatic(input) = &self;
		run(&input.data).map_err(AutomaticWebError::WebError)
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
		} else if let Some(status) = data.status {
			format!("/status/{status}")
		} else {
			"/get".to_string()
		}
	);

	reqwest::blocking::get(url.to_string())
		.and_then(|req| {
			if data.error {
				Ok(req)
			} else {
				req.error_for_status()
			}
		})
		.and_then(|req| req.json::<WebResult>())
		.map_err(|error| error.to_error(url.to_string()))
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use mockito::mock;

	use crate::{
		business::action::web_action::{
			httpbin_base_url, AutomaticWebError, ModeratorWebError, UrlData, UserWebError,
			WebActionAutomatic, WebActionModerator, WebActionUser, WebData, WebResult,
			WebResultArgs, WebSharedError,
		},
		core::action::{
			action_type::moderator_action_type::ModeratorActionType,
			data::{
				action_data::{ActionContext, ActionErrorInfo, ErrorInfo, RequestInput},
				automatic_action_data::{
					tests::{automatic_context, AutomaticTestOptions},
					AutomaticOutputInfo,
				},
				moderator_action_data::{
					tests::{moderator_context, ModeratorTestOptions},
					ModeratorOutputInfo,
				},
				user_action_data::{
					tests::{user_context, UserTestOptions},
					UserOutputInfo,
				},
			},
			definition::action::{
				Action, ActionError, AutomaticAction, ModeratorAction, UserAction,
			},
		},
		tests::test_utils::tests::run_test,
	};

	#[test]
	fn test_user_auth_ok() {
		run_test(|_| {
			let context = user_context(UserTestOptions { user_id: Some(1) });
			let action_context = ActionContext {
				action_type: WebActionUser::action_type(),
				context: context.clone(),
			};

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

			let result = WebActionUser::run(RequestInput {
				data: WebData {
					error: false,
					status: None,
				},
				context,
			});

			assert_eq!(
				&result,
				&Ok(UserOutputInfo {
					action_context,
					data: WebResult {
						url: "http://httpbin.org.mock/get".to_string(),
						args: WebResultArgs {}
					},
				}),
			);
		});
	}

	#[test]
	fn test_user_no_auth_ok() {
		run_test(|_| {
			let context = user_context(UserTestOptions { user_id: None });
			let action_context = ActionContext {
				action_type: WebActionUser::action_type(),
				context: context.clone(),
			};

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

			let result = WebActionUser::run(RequestInput {
				data: WebData {
					error: false,
					status: None,
				},
				context: context.clone(),
			});

			assert_eq!(
				&result,
				&Ok(UserOutputInfo {
					action_context,
					data: WebResult {
						url: "http://httpbin.org.mock/get".to_string(),
						args: WebResultArgs {}
					},
				}),
			);
		});
	}

	#[test]
	fn test_user_status_error() {
		run_test(|_| {
			let context = user_context(UserTestOptions { user_id: None });
			let action_context = ActionContext {
				action_type: WebActionUser::action_type(),
				context: context.clone(),
			};

			let _m = mock("GET", "/mock/http/status/403")
				.with_status(403)
				.create();

			let result = WebActionUser::run(RequestInput {
				data: WebData {
					error: false,
					status: Some(403),
				},
				context: context.clone(),
			});

			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: UserWebError::WebError(WebSharedError::Reqwest(ErrorInfo::mock(
						UrlData {
							url: format!(
								"{host}/{path}",
								host = httpbin_base_url(),
								path = "status/403"
							),
							status: Some(403)
						}
					))),
				}),
			);

			let public_error = &result.unwrap_err().error.public_error();

			assert_eq!(
				public_error.as_ref().unwrap().msg,
				"Web Action - Forbidden".to_string()
			);
		});
	}

	#[test]
	fn test_user_no_status_error() {
		run_test(|_| {
			let context = user_context(UserTestOptions { user_id: None });
			let action_context = ActionContext {
				action_type: WebActionUser::action_type(),
				context: context.clone(),
			};

			let result = WebActionUser::run(RequestInput {
				data: WebData {
					error: true,
					status: None,
				},
				context: context.clone(),
			});

			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: UserWebError::WebError(WebSharedError::Reqwest(ErrorInfo::mock(
						UrlData {
							url: format!(
								"{host}/{path}",
								host = httpbin_base_url(),
								path = "get/error"
							),
							status: None
						}
					))),
				}),
			);

			let public_error = &result.unwrap_err().error.public_error();

			assert_eq!(
				public_error.as_ref().unwrap().msg,
				"Web error occured".to_string()
			);
		});
	}

	#[test]
	fn test_mod_ok() {
		run_test(|_| {
			let context = moderator_context(ModeratorTestOptions {
				admin: false,
				allowed_actions: vec![ModeratorActionType::Web],
			});
			let action_context = ActionContext {
				action_type: WebActionModerator::action_type(),
				context: context.clone(),
			};

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

			let result = WebActionModerator::run(RequestInput {
				data: WebData {
					error: false,
					status: None,
				},
				context,
			});

			assert_eq!(
				&result,
				&Ok(ModeratorOutputInfo {
					action_context,
					data: WebResult {
						url: "http://httpbin.org.mock/get".to_string(),
						args: WebResultArgs {}
					},
				}),
			);
		});
	}

	#[test]
	fn test_mod_status_error() {
		run_test(|_| {
			let context = moderator_context(ModeratorTestOptions {
				admin: false,
				allowed_actions: vec![ModeratorActionType::Web],
			});

			let _m = mock("GET", "/mock/http/status/403")
				.with_status(403)
				.create();

			let result = WebActionModerator::run(RequestInput {
				data: WebData {
					error: false,
					status: Some(403),
				},
				context: context.clone(),
			});

			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context: ActionContext {
						action_type: WebActionModerator::action_type(),
						context: context.clone(),
					},
					error: ModeratorWebError::WebError(WebSharedError::Reqwest(ErrorInfo::mock(
						UrlData {
							url: format!(
								"{host}/{path}",
								host = httpbin_base_url(),
								path = "status/403"
							),
							status: Some(403)
						}
					))),
				}),
			);

			let public_error = &result.unwrap_err().error.public_error();

			assert_eq!(
				public_error.as_ref().unwrap().msg,
				"Web Action - Forbidden".to_string()
			);
		});
	}

	#[test]
	fn test_mod_no_status_error() {
		run_test(|_| {
			let context = moderator_context(ModeratorTestOptions {
				admin: false,
				allowed_actions: vec![ModeratorActionType::Web],
			});

			let result = WebActionModerator::run(RequestInput {
				data: WebData {
					error: true,
					status: None,
				},
				context: context.clone(),
			});

			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context: ActionContext {
						action_type: WebActionModerator::action_type(),
						context: context.clone(),
					},
					error: ModeratorWebError::WebError(WebSharedError::Reqwest(ErrorInfo::mock(
						UrlData {
							url: format!(
								"{host}/{path}",
								host = httpbin_base_url(),
								path = "get/error"
							),
							status: None
						}
					))),
				}),
			);

			let public_error = &result.unwrap_err().error.public_error();

			assert_eq!(
				public_error.as_ref().unwrap().msg,
				"Web error occured".to_string()
			);
		});
	}

	#[test]
	fn test_auto_internal_ok() {
		run_test(|_| {
			let context = automatic_context(AutomaticTestOptions { internal: true });
			let action_context = ActionContext {
				action_type: WebActionAutomatic::action_type(),
				context: context.clone(),
			};

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
				context,
			});

			assert_eq!(
				&result,
				&Ok(AutomaticOutputInfo {
					action_context,
					data: WebResult {
						url: "http://httpbin.org.mock/get".to_string(),
						args: WebResultArgs {}
					},
				})
			);
		});
	}

	#[test]
	fn test_auto_status_error() {
		run_test(|_| {
			let context = automatic_context(AutomaticTestOptions { internal: true });
			let action_context = ActionContext {
				action_type: WebActionAutomatic::action_type(),
				context: context.clone(),
			};

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
				&Err(ActionErrorInfo {
					action_context,
					error: AutomaticWebError::WebError(WebSharedError::Reqwest(ErrorInfo::mock(
						UrlData {
							url: format!(
								"{host}/{path}",
								host = httpbin_base_url(),
								path = "status/403"
							),
							status: Some(403)
						}
					))),
				}),
			);

			let public_error = &result.unwrap_err().error.public_error();

			assert_eq!(
				public_error.as_ref().unwrap().msg,
				"Web Action - Forbidden".to_string()
			);
		});
	}

	#[test]
	fn test_auto_no_status_error() {
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
				&Err(ActionErrorInfo {
					action_context: ActionContext {
						action_type: WebActionAutomatic::action_type(),
						context: context.clone(),
					},
					error: AutomaticWebError::WebError(WebSharedError::Reqwest(ErrorInfo::mock(
						UrlData {
							url: format!(
								"{host}/{path}",
								host = httpbin_base_url(),
								path = "get/error"
							),
							status: None
						}
					))),
				}),
			);

			let public_error = &result.unwrap_err().error.public_error();

			assert_eq!(
				public_error.as_ref().unwrap().msg,
				"Web error occured".to_string()
			);
		});
	}
}
