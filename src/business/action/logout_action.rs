use crate::core::action::definition::action::{ActionError, UserAction};
use crate::core::action::{
	action_type::user_action_type::UserActionType,
	data::{
		action_data::{DescriptiveError, ErrorData, RequestInput},
		user_action_data::{UserActionError, UserRequestContext},
	},
};

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum LogoutError {
	UserError(UserActionError),
}

impl ActionError for LogoutError {
	fn private_error(&self) -> DescriptiveError {
		match self {
			LogoutError::UserError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			LogoutError::UserError(error) => error.public_error(),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct LogoutAction(RequestInput<(), UserRequestContext>);

impl UserAction<(), (), LogoutError> for LogoutAction {
	fn action_type() -> UserActionType {
		UserActionType::Logout
	}

	fn new(
		input: Result<RequestInput<(), UserRequestContext>, UserActionError>,
	) -> Result<Self, LogoutError> {
		input.map(Self).map_err(LogoutError::UserError)
	}

	fn run_inner(self) -> Result<(), LogoutError> {
		println!("logout");
		Ok(())
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::LogoutAction;
	use crate::core::action::data::{
		action_data::RequestInput,
		user_action_data::tests::{user_context, UserTestOptions},
	};
	use crate::{core::action::definition::action::Action, tests::test_utils::tests::run_test};

	#[test]
	fn main() {
		run_test(|_| {
			let context = user_context(UserTestOptions { user_id: None });
			let result = LogoutAction::run(RequestInput { data: (), context });
			assert_eq!(result, Ok(()));
		});
	}
}
