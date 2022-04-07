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
pub struct EchoInfoAction(ModeratorRequestInput<()>);

impl ModeratorAction<(), (), EchoInfoError> for EchoInfoAction {
	fn action_type() -> ModeratorActionType {
		ModeratorActionType::EchoInfo
	}

	fn new(input: ModeratorActionInput<()>) -> Result<Self, EchoInfoError> {
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
	use crate::core::action::data::action_data::RequestInput;
	use crate::core::action::data::moderator_action_data::tests::ModeratorRequestContextBuilder;
	use crate::core::action::data::moderator_action_data::tests::ModeratorSessionBuilder;
	use crate::core::action::data::moderator_action_data::ModeratorActionError;
	use crate::core::action::data::moderator_action_data::ModeratorRequestContext;
	use crate::core::action::definition::action::Action;
	use crate::core::action::definition::action::ModeratorAction;
	use crate::core::action::{
		action_type::moderator_action_type::ModeratorActionType,
		data::moderator_action_data::ModeratorOutputInfo,
	};
	use crate::tests::test_utils::tests::run_test;
	use crate::{
		business::action::echo::echo_info_action::EchoInfoError,
		core::action::data::action_data::{ActionContext, ActionErrorInfo},
	};

	fn moderator_context() -> ModeratorRequestContext {
		ModeratorRequestContextBuilder::new()
			.session(
				ModeratorSessionBuilder::new()
					.allowed_actions(vec![EchoInfoAction::action_type()])
					.build(),
			)
			.build()
	}

	#[test]
	fn test_not_allowed() {
		run_test(|_| {
			let context = ModeratorRequestContextBuilder::new().build();
			let action_context = ActionContext {
				action_type: EchoInfoAction::action_type(),
				context: context.clone(),
			};

			let result = EchoInfoAction::run(RequestInput { data: (), context });
			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: EchoInfoError::ModeratorError(ModeratorActionError::NotAllowed(
						ModeratorActionType::EchoInfo
					)),
				}),
			);
		});
	}

	#[test]
	fn test_ok() {
		run_test(|helper| {
			let context = moderator_context();
			let action_context = ActionContext {
				action_type: EchoInfoAction::action_type(),
				context: context.clone(),
			};

			let result = EchoInfoAction::run(RequestInput { data: (), context });
			assert_eq!(
				&result,
				&Ok(ModeratorOutputInfo {
					action_context,
					data: (),
				}),
			);
			assert_eq!(
				helper.pop_log(),
				Some("INFO - echo info action".to_string())
			);
		});
	}

	#[test]
	fn test_ok_admin() {
		run_test(|helper| {
			let context = moderator_context();
			let action_context = ActionContext {
				action_type: EchoInfoAction::action_type(),
				context: context.clone(),
			};

			let result = EchoInfoAction::run(RequestInput { data: (), context });
			assert_eq!(
				&result,
				&Ok(ModeratorOutputInfo {
					action_context,
					data: (),
				}),
			);
			assert_eq!(
				helper.pop_log(),
				Some("INFO - echo info action".to_string())
			);
		});
	}
}
