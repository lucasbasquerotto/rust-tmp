use crate::business::{
	action_type::user_action_type::UserActionType,
	data::{
		action_data::{ErrorContext, ErrorData, RequestContext, RequestInput},
		user_action_data::{UserActionError, UserRequestContext},
	},
	definition::action::{ActionError, UserAction},
};

#[derive(Debug, PartialEq)]
pub enum LogoutError {
	UserError(UserActionError),
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
}

#[derive(Debug)]
pub struct LogoutAction<T: RequestContext>(RequestInput<(), T>);

impl UserAction<(), (), LogoutError> for LogoutAction<UserRequestContext> {
	fn action_type() -> UserActionType {
		UserActionType::Logout
	}

	fn new(
		input: Result<RequestInput<(), UserRequestContext>, UserActionError>,
	) -> Result<Self, LogoutError> {
		input
			.map(|ok_input| Self(ok_input))
			.map_err(|err| LogoutError::UserError(err))
	}

	fn run_inner(self) -> Result<(), LogoutError> {
		println!("logout");
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::LogoutAction;
	use crate::{
		business::{data::action_data::RequestInput, definition::action::Action},
		tests::test_utils::tests::{run_test, user_context, UserOptions},
	};

	#[test]
	fn main() {
		run_test(|_| {
			let context = user_context(UserOptions { user_id: None });
			let result = LogoutAction::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(result, Ok(()));
		});
	}
}
