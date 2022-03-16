use std::fmt::Debug;

use crate::{
	business::action::{
		action_log::{ActionLogger, RequestInfoDescription},
		business_action::{
			Application, BusinessActionType, BusinessException, ErrorData, Request, Session,
		},
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

impl RequestInfoDescription for UserRequestInfo {
	fn description(&self) -> String {
		let UserRequestInfo {
			action_type,
			session: Session { user_id },
			..
		} = &self;
		let action_id = action_type.id();
		format!("action({action_id}: {action_type:?}), user({user_id:?})")
	}
}

impl RequestInfo for UserRequestInfo {}

impl Exception<Option<ErrorData>> for BusinessException<UserRequestInfo> {
	fn handle(self) -> Option<ErrorData> {
		let _ = &self.error();
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

	fn validate(&self, info: UserRequestInfo) -> Result<(), BusinessException<UserRequestInfo>> {
		match self {
			UserActionType::LOGIN => validate_auth(info, false),
			UserActionType::LOGOUT => validate_auth(info, true),
		}
	}
}

#[derive(Debug)]
enum UserTypeError {
	UNAUTHENTICATED,
	AUTHENTICATED,
}

impl UserTypeError {
	fn private_error(&self) -> Option<ErrorData> {
		match self {
			UserTypeError::UNAUTHENTICATED => None,
			UserTypeError::AUTHENTICATED => None,
		}
	}

	fn public_error(&self) -> ErrorData {
		match self {
			UserTypeError::UNAUTHENTICATED => {
				self.error_msg("You must be authenticated to execute this action".to_string())
			}
			UserTypeError::AUTHENTICATED => {
				self.error_msg("You must be unauthenticated to execute this action".to_string())
			}
		}
	}

	fn error_msg(&self, msg: String) -> ErrorData {
		let key = format!("{self:?}");

		ErrorData {
			key,
			msg,
			params: None,
			meta: None,
		}
	}

	fn exception(&self, info: UserRequestInfo) -> BusinessException<UserRequestInfo> {
		BusinessException {
			info: Some(info),
			private: self.private_error(),
			public: Some(self.public_error()),
		}
	}
}

fn validate_auth(
	info: UserRequestInfo,
	authenticated: bool,
) -> Result<(), BusinessException<UserRequestInfo>> {
	match info.session.user_id {
		Some(_) => {
			if authenticated {
				Ok(())
			} else {
				Err(UserTypeError::UNAUTHENTICATED.exception(info))
			}
		}
		None => {
			if authenticated {
				Err(UserTypeError::AUTHENTICATED.exception(info))
			} else {
				Ok(())
			}
		}
	}
}
