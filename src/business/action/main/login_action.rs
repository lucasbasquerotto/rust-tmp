use crate::business::action::{
	action_type::user_action_type::UserActionType,
	data::{
		action_data::{ErrorContext, ErrorData, RequestContext, RequestInput},
		user_action_data::{UserActionError, UserNoAuthRequestContext, UserRequestContext},
	},
	definition::{
		action::{ActionError, ActionInput, ActionOutput, UserAction},
		action_helpers::ActionErrorHelper,
	},
};

#[derive(Debug, PartialEq)]
pub struct LoginData {
	pub name: String,
	pub pass: String,
}

impl ActionInput for LoginData {}

#[derive(Debug, PartialEq)]
pub struct LoginResult {
	pub id: u64,
	pub name: String,
}

impl ActionOutput for LoginResult {}

#[derive(Debug, PartialEq)]
pub enum LoginError {
	UserError(UserActionError),
}

impl ActionError<UserActionType, UserRequestContext> for LoginError {
	fn error_context(&self) -> &ErrorContext<UserActionType, UserRequestContext> {
		match &self {
			&Self::UserError(error) => error.error_context(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match &self {
			&Self::UserError(error) => error.public_error(),
		}
	}

	fn description(&self) -> String {
		self.default_description()
	}
}

#[derive(Debug)]
pub struct LoginAction<T: RequestContext>(RequestInput<LoginData, T>);

impl UserAction<LoginData, LoginResult, LoginError> for LoginAction<UserNoAuthRequestContext> {
	fn action_type() -> UserActionType {
		UserActionType::Login
	}

	fn new_inner(
		input: Result<RequestInput<LoginData, UserRequestContext>, UserActionError>,
	) -> Result<Self, LoginError> {
		match input {
			Err(err) => Err(LoginError::UserError(err)),
			Ok(ok_input) => {
				let real_input = ok_input.to_no_auth(Self::action_type());

				match real_input {
					Err(err) => Err(LoginError::UserError(err)),
					Ok(real_ok_input) => Ok(Self(real_ok_input)),
				}
			}
		}
	}

	fn run_inner(self) -> Result<LoginResult, LoginError> {
		let LoginAction(input) = &self;
		let LoginData { name, pass } = &input.data;
		println!("login: {name} ({pass})");
		let result = LoginResult {
			id: 1,
			name: name.to_string(),
		};
		Ok(result)
	}
}
