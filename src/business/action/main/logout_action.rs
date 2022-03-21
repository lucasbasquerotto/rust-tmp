use crate::business::action::{
	action_type::user_action_type::UserActionType,
	data::{
		action_data::{ErrorContext, ErrorData, RequestContext, RequestInput},
		user_action_data::{UserActionError, UserRequestContext},
	},
	definition::{
		action_error::BusinessException,
		business_action::{ActionError, UserAction},
	},
};

#[derive(Debug)]
pub struct LogoutAction<T: RequestContext>(RequestInput<(), T>);

#[derive(Debug, PartialEq)]
pub enum LogoutError {
	UserError(UserActionError),
}

impl ActionError for LogoutError {}

impl UserAction<(), (), LogoutError> for LogoutAction<UserRequestContext> {
	fn action_type() -> UserActionType {
		UserActionType::Logout
	}

	fn new_inner(
		input: Result<RequestInput<(), UserRequestContext>, UserActionError>,
	) -> Result<Self, LogoutError> {
		match input {
			Err(err) => Err(LogoutError::UserError(err)),
			Ok(ok_input) => Ok(Self(ok_input)),
		}
	}

	fn run_inner(self) -> Result<(), LogoutError> {
		println!("logout");
		Ok(())
	}
}

impl BusinessException<UserActionType, UserRequestContext> for LogoutError {
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
