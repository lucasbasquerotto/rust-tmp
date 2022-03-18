use std::fmt::Debug;

use crate::{
	business::action::{
		action_type::user_action_type::UserActionType,
		data::{
			action_data::{BusinessException, ErrorData},
			user_action_data::{
				UserAuthRequestContext, UserAuthSession, UserNoAuthRequestContext,
				UserNoAuthSession, UserRequestContext, UserSession,
			},
		},
		definition::action_error::BusinessErrorGenerator,
		definition::action_helpers::DescriptiveRequestContext,
		definition::business_action::{UserAction, UserActionResult},
	},
	lib::core::action::{Action, ActionScope, ActionType, RequestInput},
};

impl DescriptiveRequestContext for UserRequestContext {
	fn description(&self) -> String {
		let UserRequestContext {
			action_type,
			session: UserSession { user_id },
			..
		} = &self;
		let action_id = action_type.get_id();
		format!("action({action_id}: {action_type:?}), user({user_id:?})")
	}
}

#[derive(Debug)]
enum UserActionContextError {
	UNAUTHENTICATED,
	AUTHENTICATED,
}

impl BusinessErrorGenerator<UserRequestContext> for UserActionContextError {
	fn private_error(&self) -> Option<ErrorData> {
		match self {
			UserActionContextError::UNAUTHENTICATED => None,
			UserActionContextError::AUTHENTICATED => None,
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			UserActionContextError::UNAUTHENTICATED => {
				self.error_msg("You must be authenticated to execute this action.")
			}
			UserActionContextError::AUTHENTICATED => {
				self.error_msg("You can't execute this action while authenticated.")
			}
		}
	}
}

impl UserRequestContext {
	pub fn to_auth(&self) -> Result<UserAuthRequestContext, BusinessException<Self>> {
		let UserRequestContext {
			application,
			session,
			request,
			action_type,
		} = self.clone();

		match session.user_id {
			Some(user_id) => Ok(UserAuthRequestContext {
				application,
				session: UserAuthSession { user_id },
				request,
				action_type,
			}),
			None => Err(UserActionContextError::UNAUTHENTICATED.exception(self)),
		}
	}

	pub fn to_no_auth(&self) -> Result<UserNoAuthRequestContext, BusinessException<Self>> {
		let UserRequestContext {
			application,
			session,
			request,
			action_type,
		} = self.clone();

		match session.user_id {
			Some(_) => Err(UserActionContextError::AUTHENTICATED.exception(self)),
			None => Ok(UserNoAuthRequestContext {
				application,
				session: UserNoAuthSession(),
				request,
				action_type,
			}),
		}
	}
}

#[allow(dead_code)]
impl UserAuthRequestContext {
	pub fn to_general(&self) -> UserRequestContext {
		let UserAuthRequestContext {
			application,
			session,
			request,
			action_type,
		} = self.clone();

		UserRequestContext {
			application,
			session: UserSession {
				user_id: Some(session.user_id),
			},
			request,
			action_type,
		}
	}

	pub fn to_no_auth(
		&self,
	) -> Result<UserNoAuthRequestContext, BusinessException<UserRequestContext>> {
		self.to_general().to_no_auth()
	}
}

#[allow(dead_code)]
impl UserNoAuthRequestContext {
	pub fn to_general(&self) -> UserRequestContext {
		let UserNoAuthRequestContext {
			application,
			request,
			action_type,
			..
		} = self.clone();

		UserRequestContext {
			application,
			session: UserSession { user_id: None },
			request,
			action_type,
		}
	}

	pub fn to_auth(&self) -> Result<UserAuthRequestContext, BusinessException<UserRequestContext>> {
		self.to_general().to_auth()
	}
}

impl ActionType<UserRequestContext, Option<ErrorData>, BusinessException<UserRequestContext>, u32>
	for UserActionType
{
	fn scope() -> ActionScope {
		ActionScope::User
	}

	fn id(&self) -> u32 {
		self.get_id()
	}

	fn validate(
		&self,
		context: &UserRequestContext,
	) -> Result<(), BusinessException<UserRequestContext>> {
		match self {
			UserActionType::Login => context.to_no_auth().map(|_| ()),
			UserActionType::Logout => context.to_auth().map(|_| ()),
		}
	}
}

impl<I, O, T>
	Action<
		UserRequestContext,
		I,
		O,
		Option<ErrorData>,
		BusinessException<UserRequestContext>,
		u32,
		UserActionType,
	> for T
where
	I: Debug,
	O: Debug,
	T: UserAction<I, O>,
{
	fn action_type() -> UserActionType {
		Self::action_type()
	}

	fn new(input: RequestInput<I, UserRequestContext>) -> Self {
		Self::new(input)
	}

	fn input(&self) -> &RequestInput<I, UserRequestContext> {
		self.input()
	}

	fn run(self) -> UserActionResult<O> {
		Self::action_type().validate(&self.input().context)?;
		self.run_inner()
	}
}
