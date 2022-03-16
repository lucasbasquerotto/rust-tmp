use std::fmt::Debug;

use crate::{
	business::action::business_action::{
		Application, BusinessActionType, BusinessException, ErrorData, Request, Session,
	},
	lib::{
		base::action::Exception,
		core::action_core::{ActionContext, RequestInfo},
	},
};

#[derive(Debug, PartialEq, Eq)]
pub enum UserActionType {
	LOGIN,
	LOGOUT,
}

#[derive(Debug)]
pub struct UserRequestInfo {
	pub application: Application,
	pub session: Session,
	pub request: Request,
	pub action_type: UserActionType,
}

impl RequestInfo for UserRequestInfo {}

impl Exception<Option<ErrorData>> for BusinessException<UserRequestInfo> {
	fn handle(self) -> Option<ErrorData> {
		//TODO log
		println!(
			"error: info: {info:?} -> {private:?} / {public:?}",
			info = &self.info,
			private = &self.private,
			public = &self.public
		);
		self.public
	}
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

	fn validate(&self, input: UserRequestInfo) -> Result<(), BusinessException<UserRequestInfo>> {
		match self {
			UserActionType::LOGIN => validate_auth(input, false),
			UserActionType::LOGOUT => validate_auth(input, true),
		}
	}
}

fn validate_auth(
	input: UserRequestInfo,
	authenticated: bool,
) -> Result<(), BusinessException<UserRequestInfo>> {
	if authenticated == (input.session.user_id > 0) {
		Ok(())
	} else {
		Ok(())
	}
}
