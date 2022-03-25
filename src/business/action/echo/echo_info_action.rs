use crate::business::{
	action_type::moderator_action_type::ModeratorActionType,
	data::{
		action_data::{DescriptiveErrorInput, ErrorData, RequestInput},
		moderator_action_data::{ModeratorActionError, ModeratorRequestContext},
	},
	definition::action::{ActionError, ModeratorAction},
};

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum EchoInfoError {
	ModeratorError(ModeratorActionError),
}

impl ActionError<ModeratorActionType, ModeratorRequestContext> for EchoInfoError {
	fn error_input(&self) -> DescriptiveErrorInput<ModeratorActionType, ModeratorRequestContext> {
		match &self {
			&Self::ModeratorError(error) => error.error_input(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match &self {
			&Self::ModeratorError(error) => error.public_error(),
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
	use crate::business::action_type::moderator_action_type::ModeratorActionType;
	use crate::business::data::action_data::{ErrorContext, ErrorInput, RequestInput};
	use crate::business::data::moderator_action_data::tests::{
		moderator_context, ModeratorTestOptions,
	};
	use crate::business::data::moderator_action_data::ModeratorActionError;
	use crate::business::definition::action::Action;
	use crate::business::definition::action::ModeratorAction;
	use crate::tests::test_utils::tests::run_test;

	#[test]
	fn test_not_allowed() {
		run_test(|_| {
			let context = moderator_context(ModeratorTestOptions {
				admin: false,
				allowed_actions: vec![],
			});

			let result = EchoInfoAction::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(
				result,
				Err(EchoInfoError::ModeratorError(
					ModeratorActionError::NotAllowed(ErrorInput {
						error_context: ErrorContext {
							action_type: ModeratorActionType::EchoInfo,
							context: context.clone()
						},
						data: ModeratorActionType::EchoInfo,
						source: None
					})
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

			let result = EchoInfoAction::run(RequestInput {
				data: (),
				context: context.clone(),
			});
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

			let result = EchoInfoAction::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("INFO - echo info action".to_string())
			);
		});
	}
}
