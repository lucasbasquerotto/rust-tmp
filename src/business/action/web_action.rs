use crate::core::action::{
	data::automatic_action_data::AutomaticActionInput,
	definition::action::{
		ActionError, ActionInput, ActionOutput, AutomaticAction, ModeratorAction, UserAction,
	},
};
use crate::{
	core::action::{
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
	},
	lib::data::result::AsyncResult,
};

////////////////////////////////////////////////
///////////////////// TYPE /////////////////////
////////////////////////////////////////////////

const USER_ACTION_TYPE: UserActionType = UserActionType::Web;
const MODERATOR_ACTION_TYPE: ModeratorActionType = ModeratorActionType::Web;
const AUTOMATIC_ACTION_TYPE: AutomaticActionType = AutomaticActionType::Web;

////////////////////////////////////////////////
//////////////////// STATIC ////////////////////
////////////////////////////////////////////////

#[cfg(not(test))]
fn httpbin_base_url() -> String {
	"http://httpbin.org".into()
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
pub struct Input {
	pub error: bool,
	pub status: Option<u16>,
}

impl ActionInput for Input {}

////////////////////////////////////////////////
//////////////////// OUTPUT ////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq, Deserialize)]
pub struct WebResultArgs {}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Output {
	pub url: String,
	pub args: WebResultArgs,
}

impl ActionOutput for Output {}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum UserError {
	UserError(UserActionError),
	WebError(WebSharedError),
}

impl ActionError for UserError {
	fn private_error(&self) -> DescriptiveError {
		match &self {
			UserError::UserError(error) => error.private_error(),
			UserError::WebError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match &self {
			UserError::UserError(error) => error.public_error(),
			UserError::WebError(error) => error.public_error(),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct User(UserRequestInput<Input>);

impl UserAction<Input, Output, UserError> for User {
	fn action_type() -> UserActionType {
		USER_ACTION_TYPE
	}

	fn new(input: UserActionInput<Input>) -> AsyncResult<Self, UserError> {
		Box::pin(async { input.map(Self).map_err(UserError::UserError) })
	}

	fn run_inner(self) -> AsyncResult<Output, UserError> {
		Box::pin(async move {
			let Self(input) = &self;
			run(&input.data).await.map_err(UserError::WebError)
		})
	}
}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum ModeratorError {
	ModeratorError(ModeratorActionError),
	WebError(WebSharedError),
}

impl ActionError for ModeratorError {
	fn private_error(&self) -> DescriptiveError {
		match &self {
			ModeratorError::ModeratorError(error) => error.private_error(),
			ModeratorError::WebError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match &self {
			ModeratorError::ModeratorError(error) => error.public_error(),
			ModeratorError::WebError(error) => error.public_error(),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct Moderator(ModeratorRequestInput<Input>);

impl ModeratorAction<Input, Output, ModeratorError> for Moderator {
	fn action_type() -> ModeratorActionType {
		MODERATOR_ACTION_TYPE
	}

	fn new(input: ModeratorActionInput<Input>) -> AsyncResult<Self, ModeratorError> {
		Box::pin(async { input.map(Self).map_err(ModeratorError::ModeratorError) })
	}

	fn run_inner(self) -> AsyncResult<Output, ModeratorError> {
		Box::pin(async move {
			let Self(input) = &self;
			run(&input.data).await.map_err(ModeratorError::WebError)
		})
	}
}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum AutomaticError {
	AutomaticError(AutomaticActionError),
	WebError(WebSharedError),
}

impl ActionError for AutomaticError {
	fn private_error(&self) -> DescriptiveError {
		match &self {
			AutomaticError::AutomaticError(error) => error.private_error(),
			AutomaticError::WebError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match &self {
			AutomaticError::AutomaticError(error) => error.public_error(),
			AutomaticError::WebError(error) => error.public_error(),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct Automatic(AutomaticRequestInput<Input>);

impl AutomaticAction<Input, Output, AutomaticError> for Automatic {
	fn action_type() -> AutomaticActionType {
		AUTOMATIC_ACTION_TYPE
	}

	fn new(input: AutomaticActionInput<Input>) -> AsyncResult<Self, AutomaticError> {
		Box::pin(async { input.map(Self).map_err(AutomaticError::AutomaticError) })
	}

	fn run_inner(self) -> AsyncResult<Output, AutomaticError> {
		Box::pin(async move {
			let Self(input) = &self;
			run(&input.data).await.map_err(AutomaticError::WebError)
		})
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
						403 => "Web Action - Forbidden".into(),
						404 => "Web Action - Not Found".into(),
						status => format!("Web error -> Status: {status}").into(),
					},
					None => "Web error occured".into(),
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

async fn run(data: &Input) -> Result<Output, WebSharedError> {
	let url = format!(
		"{host}{suffix}",
		host = httpbin_base_url(),
		suffix = if data.error {
			"/get/error".into()
		} else if let Some(status) = data.status {
			format!("/status/{status}")
		} else {
			"/get".into()
		}
	);

	reqwest::get(&url)
		.await
		.and_then(|req| {
			if data.error {
				Ok(req)
			} else {
				req.error_for_status()
			}
		})
		.map_err(|error| error.to_error(url.to_string()))?
		.json::<Output>()
		.await
		.map_err(|error| error.to_error(url))
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use mockito::mock;

	use crate::{
		core::action::{
			data::{
				action_data::{ActionContext, ActionErrorInfo, ErrorInfo, RequestInput},
				automatic_action_data::{
					tests::AutomaticRequestContextBuilder, AutomaticOutputInfo,
				},
				moderator_action_data::{
					tests::{ModeratorRequestContextBuilder, ModeratorSessionBuilder},
					ModeratorOutputInfo, ModeratorRequestContext,
				},
				user_action_data::{tests::UserRequestContextBuilder, UserOutputInfo},
			},
			definition::action::{Action, ActionError},
		},
		tests::test_utils::tests::run_test,
	};

	fn moderator_context() -> ModeratorRequestContext {
		ModeratorRequestContextBuilder::new()
			.session(
				ModeratorSessionBuilder::new()
					.allowed_actions(vec![super::MODERATOR_ACTION_TYPE])
					.build(),
			)
			.build()
	}

	#[tokio::test]
	async fn test_user_auth_ok() {
		run_test(|_| async {
			let context = UserRequestContextBuilder::build_auth();
			let action_context = ActionContext {
				action_type: super::USER_ACTION_TYPE,
				context: Some(context.clone()),
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

			let result = super::User::run(Ok(RequestInput {
				data: super::Input {
					error: false,
					status: None,
				},
				context,
			}))
			.await;

			assert_eq!(
				&result,
				&Ok(UserOutputInfo {
					action_context,
					data: super::Output {
						url: "http://httpbin.org.mock/get".into(),
						args: super::WebResultArgs {}
					},
				}),
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_user_no_auth_ok() {
		run_test(|_| async {
			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: super::USER_ACTION_TYPE,
				context: Some(context.clone()),
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

			let result = super::User::run(Ok(RequestInput {
				data: super::Input {
					error: false,
					status: None,
				},
				context,
			}))
			.await;

			assert_eq!(
				&result,
				&Ok(UserOutputInfo {
					action_context,
					data: super::Output {
						url: "http://httpbin.org.mock/get".into(),
						args: super::WebResultArgs {}
					},
				}),
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_user_status_error() {
		run_test(|_| async {
			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: super::USER_ACTION_TYPE,
				context: Some(context.clone()),
			};

			let _m = mock("GET", "/mock/http/status/403")
				.with_status(403)
				.create();

			let result = super::User::run(Ok(RequestInput {
				data: super::Input {
					error: false,
					status: Some(403),
				},
				context,
			}))
			.await;

			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: super::UserError::WebError(super::WebSharedError::Reqwest(
						ErrorInfo::mock(super::UrlData {
							url: format!(
								"{host}/{path}",
								host = super::httpbin_base_url(),
								path = "status/403"
							),
							status: Some(403)
						})
					)),
				}),
			);

			let public_error = &result.unwrap_err().error.public_error();

			assert_eq!(
				&public_error.as_ref().unwrap().msg,
				&"Web Action - Forbidden"
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_user_no_status_error() {
		run_test(|_| async {
			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: super::USER_ACTION_TYPE,
				context: Some(context.clone()),
			};

			let result = super::User::run(Ok(RequestInput {
				data: super::Input {
					error: true,
					status: None,
				},
				context,
			}))
			.await;

			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: super::UserError::WebError(super::WebSharedError::Reqwest(
						ErrorInfo::mock(super::UrlData {
							url: format!(
								"{host}/{path}",
								host = super::httpbin_base_url(),
								path = "get/error"
							),
							status: None
						})
					)),
				}),
			);

			let public_error = &result.unwrap_err().error.public_error();

			assert_eq!(&public_error.as_ref().unwrap().msg, &"Web error occured");
		})
		.await;
	}

	#[tokio::test]
	async fn test_mod_ok() {
		run_test(|_| async {
			let context = moderator_context();
			let action_context = ActionContext {
				action_type: super::MODERATOR_ACTION_TYPE,
				context: Some(context.clone()),
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

			let result = super::Moderator::run(Ok(RequestInput {
				data: super::Input {
					error: false,
					status: None,
				},
				context,
			}))
			.await;

			assert_eq!(
				&result,
				&Ok(ModeratorOutputInfo {
					action_context,
					data: super::Output {
						url: "http://httpbin.org.mock/get".into(),
						args: super::WebResultArgs {}
					},
				}),
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_mod_status_error() {
		run_test(|_| async {
			let context = moderator_context();

			let _m = mock("GET", "/mock/http/status/403")
				.with_status(403)
				.create();

			let result = super::Moderator::run(Ok(RequestInput {
				data: super::Input {
					error: false,
					status: Some(403),
				},
				context: context.clone(),
			}))
			.await;

			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context: ActionContext {
						action_type: super::MODERATOR_ACTION_TYPE,
						context: Some(context),
					},
					error: super::ModeratorError::WebError(super::WebSharedError::Reqwest(
						ErrorInfo::mock(super::UrlData {
							url: format!(
								"{host}/{path}",
								host = super::httpbin_base_url(),
								path = "status/403"
							),
							status: Some(403)
						})
					)),
				}),
			);

			let public_error = &result.unwrap_err().error.public_error();

			assert_eq!(
				&public_error.as_ref().unwrap().msg,
				&"Web Action - Forbidden"
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_mod_no_status_error() {
		run_test(|_| async {
			let context = moderator_context();

			let result = super::Moderator::run(Ok(RequestInput {
				data: super::Input {
					error: true,
					status: None,
				},
				context: context.clone(),
			}))
			.await;

			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context: ActionContext {
						action_type: super::MODERATOR_ACTION_TYPE,
						context: Some(context),
					},
					error: super::ModeratorError::WebError(super::WebSharedError::Reqwest(
						super::ErrorInfo::mock(super::UrlData {
							url: format!(
								"{host}/{path}",
								host = super::httpbin_base_url(),
								path = "get/error"
							),
							status: None
						})
					)),
				}),
			);

			let public_error = &result.unwrap_err().error.public_error();

			assert_eq!(&public_error.as_ref().unwrap().msg, &"Web error occured");
		})
		.await;
	}

	#[tokio::test]
	async fn test_auto_internal_ok() {
		run_test(|_| async {
			let context = AutomaticRequestContextBuilder::build_internal();
			let action_context = ActionContext {
				action_type: super::AUTOMATIC_ACTION_TYPE,
				context: Some(context.clone()),
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

			let result = super::Automatic::run(Ok(RequestInput {
				data: super::Input {
					error: false,
					status: None,
				},
				context,
			}))
			.await;

			assert_eq!(
				&result,
				&Ok(AutomaticOutputInfo {
					action_context,
					data: super::Output {
						url: "http://httpbin.org.mock/get".into(),
						args: super::WebResultArgs {}
					},
				})
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_auto_status_error() {
		run_test(|_| async {
			let context = AutomaticRequestContextBuilder::build_internal();
			let action_context = ActionContext {
				action_type: super::AUTOMATIC_ACTION_TYPE,
				context: Some(context.clone()),
			};

			let _m = mock("GET", "/mock/http/status/403")
				.with_status(403)
				.create();

			let result = super::Automatic::run(Ok(RequestInput {
				data: super::Input {
					error: false,
					status: Some(403),
				},
				context,
			}))
			.await;

			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: super::AutomaticError::WebError(super::WebSharedError::Reqwest(
						ErrorInfo::mock(super::UrlData {
							url: format!(
								"{host}/{path}",
								host = super::httpbin_base_url(),
								path = "status/403"
							),
							status: Some(403)
						})
					)),
				}),
			);

			let public_error = &result.unwrap_err().error.public_error();

			assert_eq!(
				&public_error.as_ref().unwrap().msg,
				&"Web Action - Forbidden"
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_auto_no_status_error() {
		run_test(|_| async {
			let context = AutomaticRequestContextBuilder::build_internal();

			let result = super::Automatic::run(Ok(RequestInput {
				data: super::Input {
					error: true,
					status: None,
				},
				context: context.clone(),
			}))
			.await;

			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context: ActionContext {
						action_type: super::AUTOMATIC_ACTION_TYPE,
						context: Some(context),
					},
					error: super::AutomaticError::WebError(super::WebSharedError::Reqwest(
						ErrorInfo::mock(super::UrlData {
							url: format!(
								"{host}/{path}",
								host = super::httpbin_base_url(),
								path = "get/error"
							),
							status: None
						})
					)),
				}),
			);

			let public_error = &result.unwrap_err().error.public_error();

			assert_eq!(&public_error.as_ref().unwrap().msg, &"Web error occured");
		})
		.await;
	}
}
