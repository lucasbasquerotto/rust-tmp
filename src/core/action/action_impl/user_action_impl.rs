use crate::core::action::{
	data::{
		action_data::{ActionContext, DescriptiveError, ErrorData, RequestInput},
		user_action_data::{
			UserActionError, UserAuthRequestContext, UserAuthSession, UserErrorInfo,
			UserNoAuthRequestContext, UserNoAuthSession, UserOutputInfo, UserRequestContext,
			UserRequestInput, UserSession, UserUnconfirmedRequestContext, UserUnconfirmedSession,
		},
	},
	definition::action_helpers::DescriptiveInfo,
};
use crate::core::action::{
	definition::action::{Action, ActionError, UserAction},
	definition::action::{ActionInput, ActionOutput},
};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

impl<I: ActionInput> ActionInput for RequestInput<I, UserRequestContext> {}

impl DescriptiveInfo for UserAuthSession {
	fn description(&self) -> String {
		let UserAuthSession { user_id, .. } = &self;
		format!("user({user_id})")
	}
}

impl DescriptiveInfo for UserNoAuthSession {
	fn description(&self) -> String {
		"user(not authenticated)".to_string()
	}
}

impl DescriptiveInfo for UserUnconfirmedSession {
	fn description(&self) -> String {
		let UserUnconfirmedSession { user_id, .. } = &self;
		format!("user(unconfirmed - {user_id})")
	}
}

impl DescriptiveInfo for UserSession {
	fn description(&self) -> String {
		match self {
			UserSession::Auth(session) => session.description(),
			UserSession::NoAuth(session) => session.description(),
			UserSession::Unconfirmed(session) => session.description(),
		}
	}
}

impl DescriptiveInfo for UserRequestContext {
	fn description(&self) -> String {
		self.session.description()
	}
}

impl DescriptiveInfo for UserAuthRequestContext {
	fn description(&self) -> String {
		self.session.description()
	}
}

impl DescriptiveInfo for UserNoAuthRequestContext {
	fn description(&self) -> String {
		self.session.description()
	}
}

impl DescriptiveInfo for UserUnconfirmedRequestContext {
	fn description(&self) -> String {
		self.session.description()
	}
}

impl UserRequestContext {
	pub fn into_auth(self) -> Result<UserAuthRequestContext, UserActionError> {
		let UserRequestContext {
			application,
			session,
			request,
		} = self;

		match session {
			UserSession::Auth(session) => Ok(UserAuthRequestContext {
				application,
				session,
				request,
			}),
			UserSession::NoAuth(_) => Err(UserActionError::Unauthenticated),
			UserSession::Unconfirmed(_) => Err(UserActionError::Unauthenticated),
		}
	}

	pub fn into_no_auth(self) -> Result<UserNoAuthRequestContext, UserActionError> {
		let UserRequestContext {
			application,
			session,
			request,
		} = self;

		match session {
			UserSession::Auth(_) => Err(UserActionError::Authenticated),
			UserSession::NoAuth(session) => Ok(UserNoAuthRequestContext {
				application,
				session,
				request,
			}),
			UserSession::Unconfirmed(_) => Err(UserActionError::Authenticated),
		}
	}

	#[allow(dead_code)]
	pub fn into_unconfirmed(self) -> Result<UserUnconfirmedRequestContext, UserActionError> {
		let UserRequestContext {
			application,
			session,
			request,
		} = self;

		match session {
			UserSession::Auth(_) => Err(UserActionError::Authenticated),
			UserSession::NoAuth(_) => Err(UserActionError::Unauthenticated),
			UserSession::Unconfirmed(session) => Ok(UserUnconfirmedRequestContext {
				application,
				session,
				request,
			}),
		}
	}
}

impl<I> RequestInput<I, UserRequestContext> {
	#[allow(dead_code)]
	pub fn into_auth(self) -> Result<RequestInput<I, UserAuthRequestContext>, UserActionError> {
		let context = self.context.into_auth()?;
		Ok(RequestInput {
			context,
			data: self.data,
		})
	}

	pub fn into_no_auth(
		self,
	) -> Result<RequestInput<I, UserNoAuthRequestContext>, UserActionError> {
		let context = self.context.into_no_auth()?;
		Ok(RequestInput {
			context,
			data: self.data,
		})
	}
}

impl UserAuthRequestContext {
	pub fn into_general(self) -> UserRequestContext {
		let UserAuthRequestContext {
			application,
			session,
			request,
		} = self;

		UserRequestContext {
			application,
			session: UserSession::Auth(session),
			request,
		}
	}
}

impl<T> RequestInput<T, UserAuthRequestContext> {
	#[allow(dead_code)]
	pub fn into_general(self) -> RequestInput<T, UserRequestContext> {
		let context = self.context.into_general();
		RequestInput {
			context,
			data: self.data,
		}
	}
}

impl UserNoAuthRequestContext {
	pub fn into_general(self) -> UserRequestContext {
		let UserNoAuthRequestContext {
			application,
			request,
			session,
		} = self;

		UserRequestContext {
			application,
			session: UserSession::NoAuth(session),
			request,
		}
	}
}

impl<T> RequestInput<T, UserUnconfirmedRequestContext> {
	#[allow(dead_code)]
	pub fn into_general(self) -> RequestInput<T, UserRequestContext> {
		let context = self.context.into_general();
		RequestInput {
			context,
			data: self.data,
		}
	}
}

impl UserUnconfirmedRequestContext {
	pub fn into_general(self) -> UserRequestContext {
		let UserUnconfirmedRequestContext {
			application,
			request,
			session,
		} = self;

		UserRequestContext {
			application,
			session: UserSession::Unconfirmed(session),
			request,
		}
	}
}

impl<T> RequestInput<T, UserNoAuthRequestContext> {
	#[allow(dead_code)]
	pub fn into_general(self) -> RequestInput<T, UserRequestContext> {
		let context = self.context.into_general();
		RequestInput {
			context,
			data: self.data,
		}
	}
}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

impl ActionError for UserActionError {
	fn private_error(&self) -> DescriptiveError {
		match self {
			UserActionError::Authenticated => DescriptiveError::empty(),
			UserActionError::Unauthenticated => DescriptiveError::empty(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			UserActionError::Authenticated => {
				Self::error_msg("You can't execute this action while authenticated.".to_string())
			}
			UserActionError::Unauthenticated => {
				Self::error_msg("You must be authenticated to execute this action.".to_string())
			}
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

impl<I, O, E, T> Action<UserRequestInput<I>, UserOutputInfo<O>, UserErrorInfo<E>> for T
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError,
	T: UserAction<I, O, E>,
{
	fn run(input: UserRequestInput<I>) -> Result<UserOutputInfo<O>, UserErrorInfo<E>> {
		let action_context = ActionContext {
			action_type: Self::action_type(),
			context: input.context.clone(),
		};

		let result = Self::new(Ok(input)).and_then(|action| action.run_inner());

		match result {
			Ok(data) => Ok(UserOutputInfo {
				action_context,
				data,
			}),
			Err(error) => Err(UserErrorInfo {
				action_context,
				error,
			}),
		}
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use crate::core::action::data::user_action_data::{
		UserActionError, UserAuthRequestContext, UserAuthSession, UserNoAuthRequestContext,
		UserOutputInfo, UserSession, UserUnconfirmedSession,
	};
	use crate::core::action::data::{
		action_data::ActionContext,
		user_action_data::tests::{user_context, UserTestOptions},
	};
	use crate::core::action::data::{
		action_data::RequestInput, user_action_data::UserRequestContext,
	};
	use crate::core::action::definition::action::Action;
	use crate::core::action::definition::action::UserAction;
	use crate::core::action::{
		action_type::user_action_type::UserActionType, data::action_data::ActionErrorInfo,
	};
	use crate::tests::test_utils::tests::run_test;

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

		fn new(
			input: Result<RequestInput<(), UserRequestContext>, UserActionError>,
		) -> Result<Self, UserActionError> {
			let ok_input = input?;
			Ok(Self(ok_input))
		}

		fn run_inner(self) -> Result<(), UserActionError> {
			match self.0.context.session {
				UserSession::Auth(UserAuthSession { user_id, .. }) => {
					info!("user action test: {user_id}")
				}
				UserSession::Unconfirmed(UserUnconfirmedSession { user_id, .. }) => {
					info!("user action test: [unconfirmed] {user_id}")
				}
				UserSession::NoAuth(_) => info!("user action test"),
			};
			Ok(())
		}
	}

	impl UserAction<(), (), UserActionError> for TestActionNoAuth {
		fn action_type() -> UserActionType {
			UserActionType::Test
		}

		fn new(
			input: Result<RequestInput<(), UserRequestContext>, UserActionError>,
		) -> Result<Self, UserActionError> {
			match input {
				Err(err) => Err(err),
				Ok(ok_input) => {
					let real_input = ok_input.into_no_auth();

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

		fn new(
			input: Result<RequestInput<(), UserRequestContext>, UserActionError>,
		) -> Result<Self, UserActionError> {
			match input {
				Err(err) => Err(err),
				Ok(ok_input) => {
					let real_input = ok_input.into_auth();

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
	fn test_input_context_no_auth() {
		run_test(|_| {
			let context = user_context(UserTestOptions { user_id: None });
			let input = RequestInput { context, data: () };
			assert_eq!(
				Ok(input.context.clone()),
				input.into_no_auth().map(|ctx| ctx.into_general().context),
				"Test input context reversible change"
			);
		});
	}

	#[test]
	fn test_input_context_auth() {
		run_test(|_| {
			let context = user_context(UserTestOptions { user_id: Some(10) });
			let input = RequestInput { context, data: () };
			assert_eq!(
				Ok(input.context.clone()),
				input.into_auth().map(|ctx| ctx.into_general().context),
				"Test input context reversible change"
			);
		});
	}

	#[test]
	fn test_ok_no_auth() {
		run_test(|helper| {
			let context = user_context(UserTestOptions { user_id: None });
			let action_context = ActionContext {
				action_type: TestAction::action_type(),
				context: context.clone(),
			};

			assert_eq!(
				Ok(&context),
				context
					.clone()
					.into_no_auth()
					.map(|ctx| ctx.into_general())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestAction::run(RequestInput { data: (), context });
			assert_eq!(
				result,
				Ok(UserOutputInfo {
					action_context,
					data: (),
				}),
			);
			assert_eq!(
				helper.pop_log(),
				Some("INFO - user action test".to_string())
			);
		});
	}

	#[test]
	fn test_ok_auth() {
		run_test(|helper| {
			let context = user_context(UserTestOptions { user_id: Some(1) });
			let action_context = ActionContext {
				action_type: TestAction::action_type(),
				context: context.clone(),
			};

			assert_eq!(
				Ok(&context),
				context
					.clone()
					.into_auth()
					.map(|ctx| ctx.into_general())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestAction::run(RequestInput { data: (), context });
			assert_eq!(
				result,
				Ok(UserOutputInfo {
					action_context,
					data: (),
				}),
			);
			assert_eq!(
				helper.pop_log(),
				Some("INFO - user action test: 1".to_string())
			);
		});
	}

	#[test]
	fn test_no_auth_not_allowed() {
		run_test(|_| {
			let context = user_context(UserTestOptions { user_id: Some(2) });
			let action_context = ActionContext {
				action_type: TestActionNoAuth::action_type(),
				context: context.clone(),
			};

			assert_eq!(
				Ok(&context),
				context
					.clone()
					.into_auth()
					.map(|ctx| ctx.into_general())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestActionNoAuth::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(
				result,
				Err(ActionErrorInfo {
					action_context,
					error: UserActionError::Authenticated,
				})
			);
		});
	}

	#[test]
	fn test_no_auth_ok() {
		run_test(|helper| {
			let context = user_context(UserTestOptions { user_id: None });
			let action_context = ActionContext {
				action_type: TestActionNoAuth::action_type(),
				context: context.clone(),
			};

			assert_eq!(
				Ok(&context),
				context
					.clone()
					.into_no_auth()
					.map(|ctx| ctx.into_general())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestActionNoAuth::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(
				result,
				Ok(UserOutputInfo {
					action_context,
					data: (),
				}),
			);
			assert_eq!(
				helper.pop_log(),
				Some("INFO - user action test (no auth)".to_string())
			);
		});
	}

	#[test]
	fn test_auth_not_allowed() {
		run_test(|_| {
			let context = user_context(UserTestOptions { user_id: None });
			let action_context = ActionContext {
				action_type: TestActionAuth::action_type(),
				context: context.clone(),
			};

			assert_eq!(
				Ok(&context),
				context
					.clone()
					.into_no_auth()
					.map(|ctx| ctx.into_general())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestActionAuth::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(
				result,
				Err(ActionErrorInfo {
					action_context,
					error: UserActionError::Unauthenticated,
				})
			);
		});
	}

	#[test]
	fn test_auth_ok() {
		run_test(|helper| {
			let context = user_context(UserTestOptions { user_id: Some(3) });
			let action_context = ActionContext {
				action_type: TestActionAuth::action_type(),
				context: context.clone(),
			};

			assert_eq!(
				Ok(&context),
				context
					.clone()
					.into_auth()
					.map(|ctx| ctx.into_general())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestActionAuth::run(RequestInput { data: (), context });
			assert_eq!(
				result,
				Ok(UserOutputInfo {
					action_context,
					data: (),
				}),
			);
			assert_eq!(
				helper.pop_log(),
				Some("INFO - user action test (auth): 3".to_string())
			);
		});
	}
}
