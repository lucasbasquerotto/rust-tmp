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
pub enum EchoInfoError {
	ModeratorError(ModeratorActionError),
}

impl ActionError<ModeratorActionType, ModeratorRequestContext> for EchoInfoError {
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
pub struct EchoInfoAction<T: RequestContext>(RequestInput<(), T>);

impl ModeratorAction<(), (), EchoInfoError> for EchoInfoAction<ModeratorRequestContext> {
	fn action_type() -> ModeratorActionType {
		ModeratorActionType::EchoInfo
	}

	fn new_inner(
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

#[cfg(test)]
pub mod tests {
	use super::EchoInfoAction;
	use crate::business::action_type::moderator_action_type::ModeratorActionType;
	use crate::business::data::action_data::{ErrorContext, ErrorInput};
	use crate::business::data::moderator_action_data::ModeratorActionError;
	use crate::business::main::echo::echo_info_action::EchoInfoError;
	use crate::tests::test_utils::tests::{
		moderator_context, pop_log, ModeratorOptions, TestRequest,
	};
	use business::action_type::action_type::ActionType;
	use business::definition::action::ModeratorAction;

	#[test]
	fn main() {
		assert_eq!(pop_log(), None);

		let context = moderator_context(ModeratorOptions {
			allowed_actions: vec![],
		});

		let result = EchoInfoAction::test_request((), context.clone());
		assert_eq!(
			result,
			Err(EchoInfoError::ModeratorError(
				ModeratorActionError::NotAllowed(ErrorInput {
					error_context: ErrorContext {
						action_type: ModeratorActionType::EchoInfo,
						context: context.clone()
					},
					data: ModeratorActionType::EchoInfo.id()
				})
			))
		);
		assert_eq!(pop_log(), None);

		let context = moderator_context(ModeratorOptions {
			allowed_actions: vec![EchoInfoAction::action_type().id()],
		});

		let result = EchoInfoAction::test_request((), context.clone());
		assert_eq!(result, Ok(()));
		//assert_eq!(pop_log(), Some("INFO - echo info action".to_string()));
		assert_eq!(pop_log(), None);
	}
}
