use std::fmt::Debug;

use crate::{
	business::action::business_action::{
		Application, BusinessActionType, BusinessException, Request, Session,
	},
	lib::core::action_core::{ActionContext, RequestInfo},
};

#[derive(Debug)]
pub struct UserRequestInfo {
	pub application: Application,
	pub session: Session,
	pub request: Request,
}

impl RequestInfo for UserRequestInfo {}

#[derive(Debug, PartialEq, Eq)]
pub enum UserActionType {
	LOGIN,
	LOGOUT,
}

impl BusinessActionType<UserRequestInfo, u32> for UserActionType {
	fn context() -> ActionContext {
		ActionContext::USER
	}

	fn id(&self) -> u32 {
		match self {
			UserActionType::LOGIN => 1,
			UserActionType::LOGOUT => 2,
		}
	}

	fn validate(&self, input: UserRequestInfo) -> Result<(), BusinessException> {
		match self {
			UserActionType::LOGIN => validate_auth(input, false),
			UserActionType::LOGOUT => validate_auth(input, true),
		}
	}
}

fn validate_auth(input: UserRequestInfo, authenticated: bool) -> Result<(), BusinessException> {
	if authenticated == (input.session.user_id > 0) {
		Ok(())
	} else {
		Ok(())
	}
}
