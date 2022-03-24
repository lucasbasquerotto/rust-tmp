use crate::business::{
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
		action_helpers::ActionErrorHelper,
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
	fn run(input: RequestInput<I, UserRequestContext>) -> Result<O, E> {
		let action = Self::new_inner(Ok(input))?;
		action.run_inner()
	}
}

#[cfg(test)]
pub mod tests {
	use crate::business::action_type::user_action_type::UserActionType;
	use crate::business::data::action_data::{ErrorContext, ErrorInput};
	use crate::business::data::user_action_data::{
		UserActionError, UserAuthRequestContext, UserNoAuthRequestContext,
	};
	use crate::business::{
		data::{action_data::RequestInput, user_action_data::UserRequestContext},
		definition::action::UserAction,
	};
	use crate::tests::test_utils::tests::{run_test, user_context, TestRequest, UserOptions};

	#[derive(Debug)]
	pub struct TestAction(RequestInput<(), UserRequestContext>);

	#[derive(Debug)]
	pub struct TestActionNoAuth(RequestInput<(), UserNoAuthRequestContext>);

	#[derive(Debug)]
	pub struct TestActionAuth(RequestInput<(), UserAuthRequestContext>);

	impl UserAction<(), (), UserActionError> for TestAction {
		fn action_type() -> UserActionType {
			UserActionType::Test
		}

		fn new_inner(
			input: Result<RequestInput<(), UserRequestContext>, UserActionError>,
		) -> Result<Self, UserActionError> {
			let ok_input = input?;
			Ok(Self(ok_input))
		}

		fn run_inner(self) -> Result<(), UserActionError> {
			match self.0.context.session.user_id {
				Some(user_id) => info!("user action test: {user_id}"),
				None => info!("user action test"),
			};
			Ok(())
		}
	}

	impl UserAction<(), (), UserActionError> for TestActionNoAuth {
		fn action_type() -> UserActionType {
			UserActionType::Test
		}

		fn new_inner(
			input: Result<RequestInput<(), UserRequestContext>, UserActionError>,
		) -> Result<Self, UserActionError> {
			match input {
				Err(err) => Err(err),
				Ok(ok_input) => {
					let real_input = ok_input.to_no_auth(Self::action_type());

					match real_input {
						Err(err) => Err(err),
						Ok(real_ok_input) => Ok(Self(real_ok_input)),
					}
				}
			}
		}

		fn run_inner(self) -> Result<(), UserActionError> {
			info!("user action test (no auth)");
			Ok(())
		}
	}

	impl UserAction<(), (), UserActionError> for TestActionAuth {
		fn action_type() -> UserActionType {
			UserActionType::Test
		}

		fn new_inner(
			input: Result<RequestInput<(), UserRequestContext>, UserActionError>,
		) -> Result<Self, UserActionError> {
			match input {
				Err(err) => Err(err),
				Ok(ok_input) => {
					let real_input = ok_input.to_auth(Self::action_type());

					match real_input {
						Err(err) => Err(err),
						Ok(real_ok_input) => Ok(Self(real_ok_input)),
					}
				}
			}
		}

		fn run_inner(self) -> Result<(), UserActionError> {
			info!(
				"user action test (auth): {user_id}",
				user_id = self.0.context.session.user_id
			);
			Ok(())
		}
	}

	#[test]
	fn test_ok_no_auth() {
		run_test(|helper| {
			let context = user_context(UserOptions { user_id: None });

			let result = TestAction::test_request((), context.clone());
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("INFO - user action test".to_string())
			);
		});
	}

	#[test]
	fn test_ok_auth() {
		run_test(|helper| {
			let context = user_context(UserOptions { user_id: Some(1) });

			let result = TestAction::test_request((), context.clone());
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("INFO - user action test: 1".to_string())
			);
		});
	}

	#[test]
	fn test_no_auth_not_allowed() {
		run_test(|_| {
			let context = user_context(UserOptions { user_id: Some(2) });

			let result = TestActionNoAuth::test_request((), context.clone());
			assert_eq!(
				result,
				Err(UserActionError::Authenticated(ErrorInput {
					error_context: ErrorContext {
						action_type: UserActionType::Test,
						context: context.clone()
					},
					data: ()
				}))
			);
		});
	}

	#[test]
	fn test_no_auth_ok() {
		run_test(|helper| {
			let context = user_context(UserOptions { user_id: None });

			let result = TestActionNoAuth::test_request((), context.clone());
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("INFO - user action test (no auth)".to_string())
			);
		});
	}

	#[test]
	fn test_auth_not_allowed() {
		run_test(|_| {
			let context = user_context(UserOptions { user_id: None });

			let result = TestActionAuth::test_request((), context.clone());
			assert_eq!(
				result,
				Err(UserActionError::Unauthenticated(ErrorInput {
					error_context: ErrorContext {
						action_type: UserActionType::Test,
						context: context.clone()
					},
					data: ()
				}))
			);
		});
	}

	#[test]
	fn test_auth_ok() {
		run_test(|helper| {
			let context = user_context(UserOptions { user_id: Some(3) });

			let result = TestActionAuth::test_request((), context.clone());
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("INFO - user action test (auth): 3".to_string())
			);
		});
	}
}
