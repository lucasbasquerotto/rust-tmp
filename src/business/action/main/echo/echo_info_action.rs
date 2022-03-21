use crate::business::action::{
	action_type::moderator_action_type::ModeratorActionType,
	data::{
		action_data::{ErrorContext, ErrorData, RequestContext, RequestInput},
		moderator_action_data::{ModeratorActionError, ModeratorRequestContext},
	},
	definition::{
		action_error::BusinessException,
		business_action::{ActionError, ModeratorAction},
	},
};

#[derive(Debug)]
pub struct EchoInfoAction<T: RequestContext>(RequestInput<(), T>);

#[derive(Debug, PartialEq)]
pub enum EchoInfoError {
	ModeratorError(ModeratorActionError),
}

impl ActionError for EchoInfoError {}

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

impl BusinessException<ModeratorActionType, ModeratorRequestContext> for EchoInfoError {
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
