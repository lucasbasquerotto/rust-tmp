use crate::business::{
	action_type::moderator_action_type::ModeratorActionType,
	data::{
		action_data::{ErrorContext, ErrorData, RequestInput},
		moderator_action_data::{ModeratorActionError, ModeratorRequestContext},
	},
	definition::action::{ActionError, ModeratorAction},
};

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum EchoErrorError {
	ModeratorError(ModeratorActionError),
}

impl ActionError<ModeratorActionType, ModeratorRequestContext> for EchoErrorError {
	fn error_context(&self) -> &ErrorContext<ModeratorActionType, ModeratorRequestContext> {
		match &self {
			&Self::ModeratorError(error) => error.error_context(),
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
	use crate::business::action_type::moderator_action_type::ModeratorActionType;
	use crate::business::data::action_data::{ErrorContext, ErrorInput, RequestInput};
	use crate::business::data::moderator_action_data::tests::{
		moderator_context, ModeratorTestOptions,
	};
	use crate::business::data::moderator_action_data::ModeratorActionError;
	use crate::business::definition::action::Action;
	use crate::tests::test_utils::tests::run_test;
	use business::definition::action::ModeratorAction;

	#[test]
	fn test_not_allowed() {
		run_test(|_| {
			let context = moderator_context(ModeratorTestOptions {
				admin: false,
				allowed_actions: vec![],
			});

			let result = EchoErrorAction::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(
				result,
				Err(EchoErrorError::ModeratorError(
					ModeratorActionError::NotAllowed(ErrorInput {
						error_context: ErrorContext {
							action_type: ModeratorActionType::EchoError,
							context: context.clone()
						},
						data: ModeratorActionType::EchoError
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
				allowed_actions: vec![EchoErrorAction::action_type()],
			});

			let result = EchoErrorAction::run(RequestInput {
				data: (),
				context: context.clone(),
			});
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

			let result = EchoErrorAction::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("ERROR - echo error action".to_string())
			);
		});
	}
}
