use crate::business::action::{
	action_type::user_action_type::UserActionType,
	data::{
		action_data::{ErrorContext, ErrorData, RequestInput},
		user_action_data::{
			UserActionError, UserAuthRequestContext, UserAuthSession, UserErrorInput,
			UserNoAuthRequestContext, UserNoAuthSession, UserRequestContext, UserSession,
		},
	},
	definition::action::{ActionInput, ActionOutput},
	definition::action_helpers::{DescriptiveRequestContext, UserRequestContextLike},
	definition::{
		action::{Action, ActionError, UserAction},
		action_error::ActionErrorHelper,
	},
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

impl ActionError<UserActionType, UserRequestContext> for UserActionError {
	fn error_context(&self) -> &ErrorContext<UserActionType, UserRequestContext> {
		match self {
			UserActionError::Authenticated(input) => &input.error_context,
			UserActionError::Unauthenticated(input) => &input.error_context,
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			UserActionError::Authenticated(_) => {
				self.error_msg("You can't execute this action while authenticated.".to_string())
			}
			UserActionError::Unauthenticated(_) => {
				self.error_msg("You must be authenticated to execute this action.".to_string())
			}
		}
	}

	fn description(&self) -> String {
		self.default_description()
	}
}

impl UserRequestContext {
	pub fn to_auth(
		&self,
		action_type: UserActionType,
	) -> Result<UserAuthRequestContext, UserActionError> {
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
			None => Err(UserActionError::Unauthenticated(UserErrorInput {
				error_context: ErrorContext {
					action_type,
					context: self.clone(),
				},
				data: (),
			})),
		}
	}

	pub fn to_no_auth(
		&self,
		action_type: UserActionType,
	) -> Result<UserNoAuthRequestContext, UserActionError> {
		let UserRequestContext {
			application,
			session,
			request,
		} = self.clone();

		match session.user_id {
			Some(_) => Err(UserActionError::Authenticated(UserErrorInput {
				error_context: ErrorContext {
					action_type,
					context: self.clone(),
				},
				data: (),
			})),
			None => Ok(UserNoAuthRequestContext {
				application,
				session: UserNoAuthSession(),
				request,
			}),
		}
	}
}

impl<I> RequestInput<I, UserRequestContext> {
	#[allow(dead_code)]
	pub fn to_auth(
		self,
		action_type: UserActionType,
	) -> Result<RequestInput<I, UserAuthRequestContext>, UserActionError> {
		let context = self.context.to_auth(action_type)?;
		Ok(RequestInput {
			context,
			data: self.data,
		})
	}

	pub fn to_no_auth(
		self,
		action_type: UserActionType,
	) -> Result<RequestInput<I, UserNoAuthRequestContext>, UserActionError> {
		let context = self.context.to_no_auth(action_type)?;
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

impl<I: ActionInput> ActionInput for RequestInput<I, UserRequestContext> {}

impl<I, O, E, T> Action<RequestInput<I, UserRequestContext>, O, E> for T
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError<UserActionType, UserRequestContext>,
	T: UserAction<I, O, E>,
{
	fn new(input: RequestInput<I, UserRequestContext>) -> Result<Self, E> {
		Self::new_inner(Ok(input))
	}

	fn run(self) -> Result<O, E> {
		self.run_inner()
	}
}
