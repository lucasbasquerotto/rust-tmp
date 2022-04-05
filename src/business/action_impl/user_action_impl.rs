use crate::business::{
	definition::action::{Action, ActionError, UserAction},
	definition::action::{ActionInput, ActionOutput},
	definition::action_helpers::DescriptiveRequestContext,
};
use crate::data::action::{
	action_data::{DescriptiveError, ErrorData, RequestInput},
	user_action_data::{
		UserActionError, UserAuthRequestContext, UserAuthSession, UserNoAuthRequestContext,
		UserNoAuthSession, UserRequestContext, UserSession,
	},
};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

impl<I: ActionInput> ActionInput for RequestInput<I, UserRequestContext> {}

impl DescriptiveRequestContext for UserRequestContext {
	fn description(&self) -> String {
		let UserRequestContext {
			session: UserSession { user_id, .. },
			..
		} = &self;
		format!("user({user_id:?})")
	}
}

impl DescriptiveRequestContext for UserAuthRequestContext {
	fn description(&self) -> String {
		let UserAuthRequestContext {
			session: UserAuthSession { user_id, .. },
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

impl UserRequestContext {
	pub fn into_auth(self) -> Result<UserAuthRequestContext, UserActionError> {
		let UserRequestContext {
			application,
			session,
			request,
		} = self;

		match session.user_id {
			Some(user_id) => Ok(UserAuthRequestContext {
				application,
				session: UserAuthSession {
					created_at: session.created_at,
					user_id,
				},
				request,
			}),
			None => Err(UserActionError::Unauthenticated),
		}
	}

	pub fn into_no_auth(self) -> Result<UserNoAuthRequestContext, UserActionError> {
		let UserRequestContext {
			application,
			session,
			request,
		} = self;

		match session.user_id {
			Some(_) => Err(UserActionError::Authenticated),
			None => Ok(UserNoAuthRequestContext {
				application,
				session: UserNoAuthSession {
					created_at: session.created_at,
				},
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
			session: UserSession {
				created_at: session.created_at,
				user_id: Some(session.user_id),
			},
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
			session: UserSession {
				created_at: session.created_at,
				user_id: None,
			},
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

impl<I, O, E, T> Action<RequestInput<I, UserRequestContext>, O, E> for T
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError,
	T: UserAction<I, O, E>,
{
	fn run(input: RequestInput<I, UserRequestContext>) -> Result<O, E> {
		let action = Self::new(Ok(input))?;
		action.run_inner()
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use crate::business::definition::action::Action;
	use crate::business::definition::action::UserAction;
	use crate::data::action::user_action_data::tests::{user_context, UserTestOptions};
	use crate::data::action::user_action_data::{
		UserActionError, UserAuthRequestContext, UserNoAuthRequestContext,
	};
	use crate::data::action::{action_data::RequestInput, user_action_data::UserRequestContext};
	use crate::data::action_type::user_action_type::UserActionType;
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
			let context = user_context(UserTestOptions { user_id: Some(1) });

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
			let context = user_context(UserTestOptions { user_id: Some(2) });

			assert_eq!(
				Ok(&context),
				context
					.clone()
					.into_auth()
					.map(|ctx| ctx.into_general())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestActionNoAuth::run(RequestInput { data: (), context });
			assert_eq!(result, Err(UserActionError::Authenticated));
		});
	}

	#[test]
	fn test_no_auth_ok() {
		run_test(|helper| {
			let context = user_context(UserTestOptions { user_id: None });

			assert_eq!(
				Ok(&context),
				context
					.clone()
					.into_no_auth()
					.map(|ctx| ctx.into_general())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestActionNoAuth::run(RequestInput { data: (), context });
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
			let context = user_context(UserTestOptions { user_id: None });

			assert_eq!(
				Ok(&context),
				context
					.clone()
					.into_no_auth()
					.map(|ctx| ctx.into_general())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestActionAuth::run(RequestInput { data: (), context });
			assert_eq!(result, Err(UserActionError::Unauthenticated));
		});
	}

	#[test]
	fn test_auth_ok() {
		run_test(|helper| {
			let context = user_context(UserTestOptions { user_id: Some(3) });

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
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("INFO - user action test (auth): 3".to_string())
			);
		});
	}
}
