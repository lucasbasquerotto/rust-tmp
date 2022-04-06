use crate::core::action::{
	action_type::moderator_action_type::ModeratorActionType,
	data::{
		action_data::{DescriptiveError, ErrorData},
		moderator_action_data::{ModeratorActionError, ModeratorRequestInput},
	},
};
use crate::core::action::{
	data::moderator_action_data::ModeratorActionInput,
	definition::action::{ActionError, ModeratorAction},
};

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum EchoWarnError {
	ModeratorError(ModeratorActionError),
}

impl ActionError for EchoWarnError {
	fn private_error(&self) -> DescriptiveError {
		match self {
			EchoWarnError::ModeratorError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			EchoWarnError::ModeratorError(error) => error.public_error(),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct EchoWarnAction(ModeratorRequestInput<()>);

impl ModeratorAction<(), (), EchoWarnError> for EchoWarnAction {
	fn action_type() -> ModeratorActionType {
		ModeratorActionType::EchoWarn
	}

	fn new(input: ModeratorActionInput<()>) -> Result<Self, EchoWarnError> {
		match input {
			Err(err) => Err(EchoWarnError::ModeratorError(err)),
			Ok(ok_input) => Ok(Self(ok_input)),
		}
	}

	fn run_inner(self) -> Result<(), EchoWarnError> {
		warn!("echo warn action");
		Ok(())
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use super::EchoWarnAction;
	use crate::core::action::data::moderator_action_data::tests::{
		moderator_context, ModeratorTestOptions,
	};
	use crate::core::action::data::moderator_action_data::ModeratorActionError;
	use crate::core::action::data::{
		action_data::RequestInput, moderator_action_data::ModeratorOutputInfo,
	};
	use crate::core::action::definition::action::Action;
	use crate::core::action::definition::action::ModeratorAction;
	use crate::core::action::{
		action_type::moderator_action_type::ModeratorActionType, data::action_data::ActionErrorInfo,
	};
	use crate::tests::test_utils::tests::run_test;
	use crate::{
		business::action::echo::echo_warn_action::EchoWarnError,
		core::action::data::action_data::ActionContext,
	};

	#[test]
	fn test_not_allowed() {
		run_test(|_| {
			let context = moderator_context(ModeratorTestOptions {
				admin: false,
				allowed_actions: vec![],
			});
			let action_context = ActionContext {
				action_type: EchoWarnAction::action_type(),
				context: context.clone(),
			};

			let result = EchoWarnAction::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: EchoWarnError::ModeratorError(ModeratorActionError::NotAllowed(
						ModeratorActionType::EchoWarn
					)),
				}),
			);
		});
	}

	#[test]
	fn test_ok() {
		run_test(|helper| {
			let context = moderator_context(ModeratorTestOptions {
				admin: false,
				allowed_actions: vec![EchoWarnAction::action_type()],
			});
			let action_context = ActionContext {
				action_type: EchoWarnAction::action_type(),
				context: context.clone(),
			};

			let result = EchoWarnAction::run(RequestInput { data: (), context });
			assert_eq!(
				&result,
				&Ok(ModeratorOutputInfo {
					action_context,
					data: (),
				}),
			);
			assert_eq!(
				helper.pop_log(),
				Some("WARN - echo warn action".to_string())
			);
		});
	}

	#[test]
	fn test_ok_admin() {
		run_test(|helper| {
			let context = moderator_context(ModeratorTestOptions {
				admin: true,
				allowed_actions: vec![],
			});
			let action_context = ActionContext {
				action_type: EchoWarnAction::action_type(),
				context: context.clone(),
			};

			let result = EchoWarnAction::run(RequestInput { data: (), context });
			assert_eq!(
				&result,
				&Ok(ModeratorOutputInfo {
					action_context,
					data: (),
				}),
			);
			assert_eq!(
				helper.pop_log(),
				Some("WARN - echo warn action".to_string())
			);
		});
	}
}
