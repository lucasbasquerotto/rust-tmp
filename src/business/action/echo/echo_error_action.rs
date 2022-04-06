use crate::core::action::definition::action::{ActionError, ModeratorAction};
use crate::core::action::{
	action_type::moderator_action_type::ModeratorActionType,
	data::{
		action_data::{DescriptiveError, ErrorData, RequestInput},
		moderator_action_data::{ModeratorActionError, ModeratorRequestContext},
	},
};

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum EchoErrorError {
	ModeratorError(ModeratorActionError),
}

impl ActionError for EchoErrorError {
	fn private_error(&self) -> DescriptiveError {
		match self {
			EchoErrorError::ModeratorError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			EchoErrorError::ModeratorError(error) => error.public_error(),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct EchoErrorAction(RequestInput<(), ModeratorRequestContext>);

impl ModeratorAction<(), (), EchoErrorError> for EchoErrorAction {
	fn action_type() -> ModeratorActionType {
		ModeratorActionType::EchoError
	}

	fn new(
		input: Result<RequestInput<(), ModeratorRequestContext>, ModeratorActionError>,
	) -> Result<Self, EchoErrorError> {
		match input {
			Err(err) => Err(EchoErrorError::ModeratorError(err)),
			Ok(ok_input) => Ok(Self(ok_input)),
		}
	}

	fn run_inner(self) -> Result<(), EchoErrorError> {
		error!("echo error action");
		Ok(())
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use super::EchoErrorAction;
	use crate::business::action::echo::echo_error_action::EchoErrorError;
	use crate::core::action::action_type::moderator_action_type::ModeratorActionType;
	use crate::core::action::data::action_data::RequestInput;
	use crate::core::action::data::moderator_action_data::tests::{
		moderator_context, ModeratorTestOptions,
	};
	use crate::core::action::data::moderator_action_data::ModeratorActionError;
	use crate::core::action::definition::action::Action;
	use crate::core::action::definition::action::ModeratorAction;
	use crate::tests::test_utils::tests::run_test;

	#[test]
	fn test_not_allowed() {
		run_test(|_| {
			let context = moderator_context(ModeratorTestOptions {
				admin: false,
				allowed_actions: vec![],
			});

			let result = EchoErrorAction::run(RequestInput { data: (), context });
			assert_eq!(
				result,
				Err(EchoErrorError::ModeratorError(
					ModeratorActionError::NotAllowed(ModeratorActionType::EchoError)
				))
			);
		});
	}

	#[test]
	fn test_ok() {
		run_test(|helper| {
			let context = moderator_context(ModeratorTestOptions {
				admin: false,
				allowed_actions: vec![EchoErrorAction::action_type()],
			});

			let result = EchoErrorAction::run(RequestInput { data: (), context });
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("ERROR - echo error action".to_string())
			);
		});
	}

	#[test]
	fn test_ok_admin() {
		run_test(|helper| {
			let context = moderator_context(ModeratorTestOptions {
				admin: true,
				allowed_actions: vec![EchoErrorAction::action_type()],
			});

			let result = EchoErrorAction::run(RequestInput { data: (), context });
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("ERROR - echo error action".to_string())
			);
		});
	}
}
