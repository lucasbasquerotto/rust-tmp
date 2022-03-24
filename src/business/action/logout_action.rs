use crate::business::{
	action_type::user_action_type::UserActionType,
	data::{
		action_data::{ErrorContext, ErrorData, RequestContext, RequestInput},
		user_action_data::{UserActionError, UserRequestContext},
	},
	definition::{
		action::{ActionError, UserAction},
		action_helpers::ActionErrorHelper,
	},
};

#[derive(Debug)]
pub struct LogoutAction<T: RequestContext>(RequestInput<(), T>);

#[derive(Debug, PartialEq)]
pub enum LogoutError {
	UserError(UserActionError),
}

impl UserAction<(), (), LogoutError> for LogoutAction<UserRequestContext> {
	fn action_type() -> UserActionType {
		UserActionType::Logout
	}

	fn new(
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

impl ActionError<UserActionType, UserRequestContext> for LogoutError {
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

#[cfg(test)]
mod tests {
	use super::LogoutAction;
	use crate::tests::test_utils::tests::{run_test, user_context, TestRequest, UserOptions};

	#[test]
	fn main() {
		run_test(|_| {
			let context = user_context(UserOptions { user_id: None });
			let result = LogoutAction::test_request((), context);
			assert_eq!(result, Ok(()));
		});
	}
}
