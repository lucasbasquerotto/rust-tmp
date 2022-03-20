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
		definition::action_helpers::{DescriptiveRequestContext, UserRequestContextLike},
		definition::business_action::{UserAction, UserActionResult},
		definition::{
			action_error::BusinessErrorGenerator,
			business_action::{ActionInput, ActionOutput},
		},
	},
	lib::core::action::{Action, RequestInput},
};

impl DescriptiveRequestContext for UserRequestContext {
	fn description(&self) -> String {
		let UserRequestContext {
			session: UserSession { user_id },
			..
		} = &self;
		format!("user({user_id:?})")
	}
}

impl DescriptiveRequestContext for UserAuthRequestContext {
	fn description(&self) -> String {
		let UserAuthRequestContext {
			session: UserAuthSession { user_id },
			..
		} = &self;
		format!("user({user_id:?})")
	}
}

impl DescriptiveRequestContext for UserNoAuthRequestContext {
	fn description(&self) -> String {
		"unauthenticated".to_string()
	}
}

#[derive(Debug)]
enum UserActionContextError {
	Authenticated,
	Unauthenticated,
}

impl BusinessErrorGenerator<UserRequestContext> for UserActionContextError {
	fn private_error(&self) -> Option<ErrorData> {
		match self {
			UserActionContextError::Unauthenticated => None,
			UserActionContextError::Authenticated => None,
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			UserActionContextError::Authenticated => {
				self.error_msg("You can't execute this action while authenticated.".to_string())
			}
			UserActionContextError::Unauthenticated => {
				self.error_msg("You must be authenticated to execute this action.".to_string())
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
		} = self.clone();

		match session.user_id {
			Some(user_id) => Ok(UserAuthRequestContext {
				application,
				session: UserAuthSession { user_id },
				request,
			}),
			None => Err(UserActionContextError::Unauthenticated.exception(self)),
		}
	}

	pub fn to_no_auth(&self) -> Result<UserNoAuthRequestContext, BusinessException<Self>> {
		let UserRequestContext {
			application,
			session,
			request,
		} = self.clone();

		match session.user_id {
			Some(_) => Err(UserActionContextError::Authenticated.exception(self)),
			None => Ok(UserNoAuthRequestContext {
				application,
				session: UserNoAuthSession(),
				request,
			}),
		}
	}
}

impl<T> RequestInput<T, UserRequestContext> {
	#[allow(dead_code)]
	pub fn to_auth(self) -> UserActionResult<RequestInput<T, UserAuthRequestContext>> {
		let context = self.context.to_auth()?;
		Ok(RequestInput {
			context,
			data: self.data,
		})
	}

	pub fn to_no_auth(self) -> UserActionResult<RequestInput<T, UserNoAuthRequestContext>> {
		let context = self.context.to_no_auth()?;
		Ok(RequestInput {
			context,
			data: self.data,
		})
	}
}

#[allow(dead_code)]
impl UserAuthRequestContext {
	pub fn to_general(&self) -> UserRequestContext {
		let UserAuthRequestContext {
			application,
			session,
			request,
		} = self.clone();

		UserRequestContext {
			application,
			session: UserSession {
				user_id: Some(session.user_id),
			},
			request,
		}
	}
}

impl<T> RequestInput<T, UserAuthRequestContext> {
	#[allow(dead_code)]
	pub fn to_general(self) -> RequestInput<T, UserRequestContext> {
		let context = self.context.to_general();
		RequestInput {
			context,
			data: self.data,
		}
	}
}

impl UserRequestContextLike for UserAuthRequestContext {
	fn user_context(&self) -> UserRequestContext {
		self.to_general()
	}
}

#[allow(dead_code)]
impl UserNoAuthRequestContext {
	pub fn to_general(&self) -> UserRequestContext {
		let UserNoAuthRequestContext {
			application,
			request,
			..
		} = self.clone();

		UserRequestContext {
			application,
			session: UserSession { user_id: None },
			request,
		}
	}
}

impl UserRequestContextLike for UserNoAuthRequestContext {
	fn user_context(&self) -> UserRequestContext {
		self.to_general()
	}
}

impl<T> RequestInput<T, UserNoAuthRequestContext> {
	#[allow(dead_code)]
	pub fn to_general(self) -> RequestInput<T, UserRequestContext> {
		let context = self.context.to_general();
		RequestInput {
			context,
			data: self.data,
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
		UserActionType,
	> for T
where
	I: ActionInput,
	O: ActionOutput,
	T: UserAction<I, O>,
{
	fn action_type() -> UserActionType {
		Self::action_type()
	}

	fn new(input: RequestInput<I, UserRequestContext>) -> UserActionResult<Self> {
		Self::new(input)
	}

	fn run(self) -> UserActionResult<O> {
		self.run_inner()
	}
}
