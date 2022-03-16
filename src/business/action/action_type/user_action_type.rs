use std::fmt::Debug;

use crate::{
	business::action::{
		action_data::{Application, BusinessException, ErrorData, Request, Session},
		action_log::{ActionLogger, DescriptiveRequestContext},
		business_action::BusinessActionType,
	},
	lib::{
		core::action::Exception,
		core::action::{ActionScope, RequestContext},
	},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserActionType {
	LOGIN,
	LOGOUT,
}

#[derive(Clone, Debug)]
pub struct UserRequestContext {
	pub application: Application,
	pub session: Session,
	pub request: Request,
	pub action_type: UserActionType,
}

impl DescriptiveRequestContext for UserRequestContext {
	fn description(&self) -> String {
		let UserRequestContext {
			action_type,
			session: Session { user_id },
			..
		} = &self;
		let action_id = action_type.id();
		format!("action({action_id}: {action_type:?}), user({user_id:?})")
	}
}

impl RequestContext for UserRequestContext {}

impl Exception<Option<ErrorData>> for BusinessException<UserRequestContext> {
	fn handle(self) -> Option<ErrorData> {
		let _ = &self.error();
		self.public
	}
}

impl BusinessActionType<UserRequestContext, u32> for UserActionType {
	fn scope() -> ActionScope {
		ActionScope::USER
	}

	fn id(&self) -> u32 {
		match self {
			UserActionType::LOGIN => 1,
			UserActionType::LOGOUT => 2,
		}
	}

	fn validate(
		&self,
		context: &UserRequestContext,
	) -> Result<(), BusinessException<UserRequestContext>> {
		match self {
			UserActionType::LOGIN => validate_auth(context, false),
			UserActionType::LOGOUT => validate_auth(context, true),
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
				self.error_msg("You must be authenticated to execute this action.")
			}
			UserTypeError::AUTHENTICATED => {
				self.error_msg("You can't execute this action while authenticated.")
			}
		}
	}

	fn error_msg(&self, msg: &'static str) -> ErrorData {
		let key = format!("{self:?}");

		ErrorData {
			key,
			msg,
			params: None,
			meta: None,
		}
	}

	fn exception(&self, context: &UserRequestContext) -> BusinessException<UserRequestContext> {
		BusinessException {
			context: Some(context.clone()),
			private: self.private_error(),
			public: Some(self.public_error()),
		}
	}
}

fn validate_auth(
	context: &UserRequestContext,
	expect_auth: bool,
) -> Result<(), BusinessException<UserRequestContext>> {
	match context.session.user_id {
		Some(_) => {
			if expect_auth {
				Ok(())
			} else {
				Err(UserTypeError::AUTHENTICATED.exception(context))
			}
		}
		None => {
			if expect_auth {
				Err(UserTypeError::UNAUTHENTICATED.exception(context))
			} else {
				Ok(())
			}
		}
	}
}
