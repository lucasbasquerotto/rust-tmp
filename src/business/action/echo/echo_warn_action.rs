use crate::business::{
	action_type::moderator_action_type::ModeratorActionType,
	data::{
		action_data::{ErrorContext, ErrorData, RequestContext, RequestInput},
		moderator_action_data::{ModeratorActionError, ModeratorRequestContext},
	},
	definition::{
		action::{ActionError, ModeratorAction},
		action_helpers::ActionErrorHelper,
	},
};

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

	fn description(&self) -> String {
		self.default_description()
	}
}

#[derive(Debug)]
pub struct EchoWarnAction<T: RequestContext>(RequestInput<(), T>);

impl ModeratorAction<(), (), EchoWarnError> for EchoWarnAction<ModeratorRequestContext> {
	fn action_type() -> ModeratorActionType {
		ModeratorActionType::EchoWarn
	}

	fn new_inner(
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

#[cfg(test)]
pub mod tests {
	use super::EchoWarnAction;
	use crate::business::action::echo::echo_warn_action::EchoWarnError;
	use crate::business::action_type::moderator_action_type::ModeratorActionType;
	use crate::business::data::action_data::{ErrorContext, ErrorInput};
	use crate::business::data::moderator_action_data::ModeratorActionError;
	use crate::tests::test_utils::tests::{
		moderator_context, run_test, ModeratorOptions, TestRequest,
	};
	use business::action_type::action_type::ActionType;
	use business::definition::action::ModeratorAction;

	#[test]
	fn test_not_allowed() {
		run_test(|_| {
			let context = moderator_context(ModeratorOptions {
				allowed_actions: vec![],
			});

			let result = EchoWarnAction::test_request((), context.clone());
			assert_eq!(
				result,
				Err(EchoWarnError::ModeratorError(
					ModeratorActionError::NotAllowed(ErrorInput {
						error_context: ErrorContext {
							action_type: ModeratorActionType::EchoWarn,
							context: context.clone()
						},
						data: ModeratorActionType::EchoWarn.id()
					})
				))
			);
		});
	}

	#[test]
	fn test_ok() {
		run_test(|helper| {
			let context = moderator_context(ModeratorOptions {
				allowed_actions: vec![EchoWarnAction::action_type().id()],
			});

			let result = EchoWarnAction::test_request((), context.clone());
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("WARN - echo warn action".to_string())
			);
		});
	}
}
