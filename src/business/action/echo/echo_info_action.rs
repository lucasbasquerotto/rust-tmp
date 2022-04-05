use crate::business::definition::action::{ActionError, ModeratorAction};
use crate::data::{
	action::{
		action_data::{DescriptiveError, ErrorData, RequestInput},
		moderator_action_data::{ModeratorActionError, ModeratorRequestContext},
	},
	action_type::moderator_action_type::ModeratorActionType,
};

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum EchoInfoError {
	ModeratorError(ModeratorActionError),
}

impl ActionError for EchoInfoError {
	fn private_error(&self) -> DescriptiveError {
		match self {
			EchoInfoError::ModeratorError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			EchoInfoError::ModeratorError(error) => error.public_error(),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct EchoInfoAction(RequestInput<(), ModeratorRequestContext>);

impl ModeratorAction<(), (), EchoInfoError> for EchoInfoAction {
	fn action_type() -> ModeratorActionType {
		ModeratorActionType::EchoInfo
	}

	fn new(
		input: Result<RequestInput<(), ModeratorRequestContext>, ModeratorActionError>,
	) -> Result<Self, EchoInfoError> {
		match input {
			Err(err) => Err(EchoInfoError::ModeratorError(err)),
			Ok(ok_input) => Ok(Self(ok_input)),
		}
	}

	fn run_inner(self) -> Result<(), EchoInfoError> {
		info!("echo info action");
		Ok(())
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use super::EchoInfoAction;
	use crate::business::action::echo::echo_info_action::EchoInfoError;
	use crate::business::definition::action::Action;
	use crate::business::definition::action::ModeratorAction;
	use crate::data::action::action_data::RequestInput;
	use crate::data::action::moderator_action_data::tests::{
		moderator_context, ModeratorTestOptions,
	};
	use crate::data::action::moderator_action_data::ModeratorActionError;
	use crate::data::action_type::moderator_action_type::ModeratorActionType;
	use crate::tests::test_utils::tests::run_test;

	#[test]
	fn test_not_allowed() {
		run_test(|_| {
			let context = moderator_context(ModeratorTestOptions {
				admin: false,
				allowed_actions: vec![],
			});

			let result = EchoInfoAction::run(RequestInput { data: (), context });
			assert_eq!(
				result,
				Err(EchoInfoError::ModeratorError(
					ModeratorActionError::NotAllowed(ModeratorActionType::EchoInfo)
				))
			);
		});
	}

	#[test]
	fn test_ok() {
		run_test(|helper| {
			let context = moderator_context(ModeratorTestOptions {
				admin: false,
				allowed_actions: vec![EchoInfoAction::action_type()],
			});

			let result = EchoInfoAction::run(RequestInput { data: (), context });
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("INFO - echo info action".to_string())
			);
		});
	}

	#[test]
	fn test_ok_admin() {
		run_test(|helper| {
			let context = moderator_context(ModeratorTestOptions {
				admin: true,
				allowed_actions: vec![EchoInfoAction::action_type()],
			});

			let result = EchoInfoAction::run(RequestInput { data: (), context });
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("INFO - echo info action".to_string())
			);
		});
	}
}
