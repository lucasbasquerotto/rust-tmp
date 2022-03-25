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
pub enum EchoWarnError {
	ModeratorError(ModeratorActionError),
}

impl ActionError<ModeratorActionType, ModeratorRequestContext> for EchoWarnError {
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
pub struct EchoWarnAction(RequestInput<(), ModeratorRequestContext>);

impl ModeratorAction<(), (), EchoWarnError> for EchoWarnAction {
	fn action_type() -> ModeratorActionType {
		ModeratorActionType::EchoWarn
	}

	fn new(
		input: Result<RequestInput<(), ModeratorRequestContext>, ModeratorActionError>,
	) -> Result<Self, EchoWarnError> {
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
	use crate::business::action::echo::echo_warn_action::EchoWarnError;
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

			let result = EchoWarnAction::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(
				result,
				Err(EchoWarnError::ModeratorError(
					ModeratorActionError::NotAllowed(ErrorInput {
						error_context: ErrorContext {
							action_type: ModeratorActionType::EchoWarn,
							context: context.clone()
						},
						data: ModeratorActionType::EchoWarn,
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
				allowed_actions: vec![EchoWarnAction::action_type()],
			});

			let result = EchoWarnAction::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(result, Ok(()));
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

			let result = EchoWarnAction::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("WARN - echo warn action".to_string())
			);
		});
	}
}
