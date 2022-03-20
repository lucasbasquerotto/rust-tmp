use crate::{
	business::action::{
		action_type::moderator_action_type::ModeratorActionType,
		data::{
			action_data::{ErrorContext, ErrorData},
			moderator_action_data::{ModeratorActionError, ModeratorRequestContext},
		},
		definition::{
			action_error::BusinessException,
			business_action::{ActionError, ModeratorAction},
		},
	},
	lib::core::action::{RequestContext, RequestInput},
};

#[derive(Debug)]
pub struct EchoErrorAction<T: RequestContext>(RequestInput<(), T>);

#[derive(Debug, PartialEq)]
pub enum EchoErrorError {
	ModeratorError(ModeratorActionError),
}

impl ActionError for EchoErrorError {}

impl ModeratorAction<(), (), EchoErrorError> for EchoErrorAction<ModeratorRequestContext> {
	fn action_type() -> ModeratorActionType {
		ModeratorActionType::EchoError
	}

	fn new_inner(
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

impl BusinessException<ModeratorActionType, ModeratorRequestContext> for EchoErrorError {
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
