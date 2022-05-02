use std::borrow::Cow;

use chrono::Utc;

use crate::{
	core::action::{
		data::{
			action_data::{ActionContext, DescriptiveError, ErrorData},
			user_action_data::{
				UserActionError, UserAuthRequestContext, UserAuthSession, UserErrorInfo,
				UserNoAuthRequestContext, UserOutputInfo, UserRequestContext, UserSession,
				UserUnconfirmedRequestContext, UserUnconfirmedSession,
			},
		},
		definition::action_helpers::DescriptiveInfo,
	},
	lib::data::result::AsyncResult,
};
use crate::{
	core::action::{
		data::{
			action_data::{Application, Request, RequestBasicData},
			user_action_data::{
				UserActionInput, UserAuthInputResult, UserAuthRequestInput, UserNoAuthInputResult,
				UserNoAuthRequestInput, UserNoAuthSession, UserRequestInput,
				UserUnconfirmedInputResult, UserUnconfirmedRequestInput,
			},
		},
		definition::action::{Action, ActionError, UserAction},
		definition::action::{ActionInput, ActionOutput},
	},
	lib::traits::async_from::AsyncFrom,
};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

impl<I: ActionInput> ActionInput for UserRequestInput<I> {}

impl DescriptiveInfo for UserSession {
	fn description(&self) -> Cow<'_, str> {
		match self {
			UserSession::Auth(UserAuthSession { user_id, .. }) => format!("user({user_id})").into(),
			UserSession::NoAuth(_) => "user(not authenticated)".into(),
			UserSession::Unconfirmed(UserUnconfirmedSession { user_id, .. }) => {
				format!("user(unconfirmed - {user_id})").into()
			}
		}
	}
}

impl DescriptiveInfo for UserRequestContext {
	fn description(&self) -> Cow<'_, str> {
		self.session.description()
	}
}

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

#[rocket::async_trait]
impl<I: Send> AsyncFrom<RequestBasicData<I>> for Result<UserRequestInput<I>, UserActionError> {
	async fn from(input: RequestBasicData<I>) -> Self {
		Ok(UserRequestInput {
			data: input.data,
			context: UserRequestContext {
				application: Application {
					request_timeout: 1000,
				},
				session: UserSession::NoAuth(UserNoAuthSession {
					created_at: Utc::now(),
				}),
				request: Request {
					ip: "1.2.3.4".into(),
				},
			},
		})
	}
}

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

impl From<UserRequestContext> for Result<UserNoAuthRequestContext, UserActionError> {
	fn from(from: UserRequestContext) -> Self {
		let UserRequestContext {
			application,
			session,
			request,
		} = from;

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
}

impl From<UserNoAuthRequestContext> for UserRequestContext {
	fn from(from: UserNoAuthRequestContext) -> Self {
		let UserNoAuthRequestContext {
			application,
			request,
			session,
		} = from;

		Self {
			application,
			session: UserSession::NoAuth(session),
			request,
		}
	}
}

impl<I> From<UserRequestInput<I>> for UserNoAuthInputResult<I> {
	fn from(from: UserRequestInput<I>) -> Self {
		let context: Result<UserNoAuthRequestContext, UserActionError> = from.context.into();
		let context = context?;
		Ok(UserNoAuthRequestInput {
			context,
			data: from.data,
		})
	}
}

impl<T> From<UserNoAuthRequestInput<T>> for UserRequestInput<T> {
	fn from(from: UserNoAuthRequestInput<T>) -> Self {
		let context = from.context.into();
		Self {
			context,
			data: from.data,
		}
	}
}

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

impl From<UserRequestContext> for Result<UserUnconfirmedRequestContext, UserActionError> {
	fn from(from: UserRequestContext) -> Self {
		let UserRequestContext {
			application,
			session,
			request,
		} = from;

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

impl From<UserUnconfirmedRequestContext> for UserRequestContext {
	fn from(from: UserUnconfirmedRequestContext) -> Self {
		let UserUnconfirmedRequestContext {
			application,
			request,
			session,
		} = from;

		Self {
			application,
			session: UserSession::Unconfirmed(session),
			request,
		}
	}
}

impl<I> From<UserRequestInput<I>> for UserUnconfirmedInputResult<I> {
	fn from(from: UserRequestInput<I>) -> Self {
		let context: Result<UserUnconfirmedRequestContext, UserActionError> = from.context.into();
		let context = context?;
		Ok(UserUnconfirmedRequestInput {
			context,
			data: from.data,
		})
	}
}

impl<T> From<UserUnconfirmedRequestInput<T>> for UserRequestInput<T> {
	fn from(from: UserUnconfirmedRequestInput<T>) -> Self {
		let context = from.context.into();
		Self {
			context,
			data: from.data,
		}
	}
}

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

impl From<UserRequestContext> for Result<UserAuthRequestContext, UserActionError> {
	fn from(from: UserRequestContext) -> Self {
		let UserRequestContext {
			application,
			session,
			request,
		} = from;

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
}

impl From<UserAuthRequestContext> for UserRequestContext {
	fn from(from: UserAuthRequestContext) -> Self {
		let UserAuthRequestContext {
			application,
			session,
			request,
		} = from;

		Self {
			application,
			session: UserSession::Auth(session),
			request,
		}
	}
}

impl<I> From<UserRequestInput<I>> for UserAuthInputResult<I> {
	fn from(from: UserRequestInput<I>) -> Self {
		let context: Result<UserAuthRequestContext, UserActionError> = from.context.into();
		let context = context?;
		Ok(UserAuthRequestInput {
			context,
			data: from.data,
		})
	}
}

impl<T> From<UserAuthRequestInput<T>> for UserRequestInput<T> {
	fn from(from: UserAuthRequestInput<T>) -> Self {
		let context = from.context.into();
		Self {
			context,
			data: from.data,
		}
	}
}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

impl ActionError for UserActionError {
	fn private_error(&self) -> Option<DescriptiveError> {
		match self {
			UserActionError::Authenticated => None,
			UserActionError::Unauthenticated => None,
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			UserActionError::Authenticated => {
				Self::error_msg("You can't execute this action while authenticated.".into())
			}
			UserActionError::Unauthenticated => {
				Self::error_msg("You must be authenticated to execute this action.".into())
			}
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

impl<I: 'static, O, E, T> Action<UserActionInput<I>, UserOutputInfo<O>, UserErrorInfo<E>> for T
where
	I: ActionInput + Send,
	O: ActionOutput,
	E: ActionError + From<UserActionError> + Send,
	T: UserAction<I, O, E> + Send,
{
	fn run(input: UserActionInput<I>) -> AsyncResult<UserOutputInfo<O>, UserErrorInfo<E>> {
		Box::pin(async {
			let context = input
				.as_ref()
				.map(|ok_input| Some(ok_input.context.clone()))
				.unwrap_or(None);

			let action_context = ActionContext {
				action_type: Self::action_type(),
				context,
			};

			match input {
				Ok(ok_input) => {
					let action_result = Self::new(ok_input).await;

					match action_result {
						Ok(action) => {
							let result = action.run_inner().await;

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
						Err(error) => Err(UserErrorInfo {
							action_context,
							error,
						}),
					}
				}
				Err(error) => Err(UserErrorInfo {
					action_context,
					error: E::from(error),
				}),
			}
		})
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use crate::core::action::data::action_data::ActionContext;
	use crate::core::action::data::user_action_data::tests::{
		UserAuthSessionBuilder, UserRequestContextBuilder, UserUnconfirmedSessionBuilder,
	};
	use crate::core::action::data::user_action_data::{
		UserActionError, UserAuthRequestContext, UserAuthSession, UserNoAuthRequestContext,
		UserOutputInfo, UserSession, UserUnconfirmedRequestContext, UserUnconfirmedSession,
	};
	use crate::core::action::data::{
		action_data::RequestInput, user_action_data::UserRequestContext,
	};
	use crate::core::action::definition::action::Action;
	use crate::core::action::definition::action::UserAction;
	use crate::core::action::{
		action_type::user_action_type::UserActionType, data::action_data::ActionErrorInfo,
	};
	use crate::lib::data::result::AsyncResult;
	use crate::tests::test_utils::tests::run_test;

	#[derive(Debug)]
	pub struct TestAction(RequestInput<(), UserRequestContext>);

	#[derive(Debug)]
	pub struct TestActionNoAuth(RequestInput<(), UserNoAuthRequestContext>);

	#[derive(Debug)]
	pub struct TestActionAuth(RequestInput<(), UserAuthRequestContext>);

	#[derive(Debug)]
	pub struct TestActionUnconfirmed(RequestInput<(), UserUnconfirmedRequestContext>);

	impl UserAction<(), (), UserActionError> for TestAction {
		fn action_type() -> UserActionType {
			UserActionType::Test
		}

		fn new(input: RequestInput<(), UserRequestContext>) -> AsyncResult<Self, UserActionError> {
			Box::pin(async { Ok(Self(input)) })
		}

		fn run_inner(self) -> AsyncResult<(), UserActionError> {
			Box::pin(async move {
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
			})
		}
	}

	impl UserAction<(), (), UserActionError> for TestActionNoAuth {
		fn action_type() -> UserActionType {
			UserActionType::Test
		}

		fn new(input: RequestInput<(), UserRequestContext>) -> AsyncResult<Self, UserActionError> {
			Box::pin(async {
				let real_input = input.into();

				match real_input {
					Err(err) => Err(err),
					Ok(ok_input) => Ok(Self(ok_input)),
				}
			})
		}

		fn run_inner(self) -> AsyncResult<(), UserActionError> {
			Box::pin(async {
				info!("user action test (no auth)");
				Ok(())
			})
		}
	}

	impl UserAction<(), (), UserActionError> for TestActionAuth {
		fn action_type() -> UserActionType {
			UserActionType::Test
		}

		fn new(input: RequestInput<(), UserRequestContext>) -> AsyncResult<Self, UserActionError> {
			Box::pin(async {
				let real_input = input.into();

				match real_input {
					Err(err) => Err(err),
					Ok(ok_input) => Ok(Self(ok_input)),
				}
			})
		}

		fn run_inner(self) -> AsyncResult<(), UserActionError> {
			Box::pin(async move {
				info!(
					"user action test (auth): {user_id}",
					user_id = self.0.context.session.user_id
				);
				Ok(())
			})
		}
	}

	impl UserAction<(), (), UserActionError> for TestActionUnconfirmed {
		fn action_type() -> UserActionType {
			UserActionType::Test
		}

		fn new(input: RequestInput<(), UserRequestContext>) -> AsyncResult<Self, UserActionError> {
			Box::pin(async {
				let real_input = input.into();

				match real_input {
					Err(err) => Err(err),
					Ok(ok_input) => Ok(Self(ok_input)),
				}
			})
		}

		fn run_inner(self) -> AsyncResult<(), UserActionError> {
			Box::pin(async move {
				info!(
					"user action test (unconfirmed): {user_id}",
					user_id = self.0.context.session.user_id
				);
				Ok(())
			})
		}
	}

	#[tokio::test]
	async fn test_input_context_no_auth() {
		run_test(|_| async {
			let context = UserRequestContextBuilder::build_no_auth();
			let input = RequestInput { context, data: () };
			assert_eq!(
				&Ok(input.context.clone()),
				&Result::<RequestInput<_, UserNoAuthRequestContext>, UserActionError>::from(input)
					.map(|ctx| RequestInput::<_, UserRequestContext>::from(ctx).context),
				"Test input context reversible change"
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_input_context_auth() {
		run_test(|_| async {
			let context = UserRequestContextBuilder::build_auth();
			let input = RequestInput { context, data: () };
			assert_eq!(
				&Ok(input.context.clone()),
				&Result::<RequestInput<_, UserAuthRequestContext>, UserActionError>::from(input)
					.map(|ctx| RequestInput::<_, UserRequestContext>::from(ctx).context),
				"Test input context reversible change"
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_ok_no_auth() {
		run_test(|helper| async move {
			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: TestAction::action_type(),
				context: Some(context.clone()),
			};

			assert_eq!(
				&Ok(&context),
				&Result::<UserNoAuthRequestContext, UserActionError>::from(context.clone())
					.map(|ctx| ctx.into())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestAction::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Ok(UserOutputInfo {
					action_context,
					data: (),
				}),
			);
			assert_eq!(&helper.pop_log(), &Some("INFO - user action test".into()));
		})
		.await;
	}

	#[tokio::test]
	async fn test_ok_auth() {
		run_test(|helper| async move {
			let context = UserRequestContextBuilder::new()
				.session(UserSession::Auth(
					UserAuthSessionBuilder::new().user_id(1).build(),
				))
				.build();
			let action_context = ActionContext {
				action_type: TestAction::action_type(),
				context: Some(context.clone()),
			};

			assert_eq!(
				&Ok(&context),
				&Result::<UserAuthRequestContext, UserActionError>::from(context.clone())
					.map(|ctx| ctx.into())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestAction::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Ok(UserOutputInfo {
					action_context,
					data: (),
				}),
			);
			assert_eq!(
				&helper.pop_log(),
				&Some("INFO - user action test: 1".into())
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_no_auth_not_allowed_authenticated() {
		run_test(|_| async {
			let context = UserRequestContextBuilder::build_auth();
			let action_context = ActionContext {
				action_type: TestActionNoAuth::action_type(),
				context: Some(context.clone()),
			};

			assert_eq!(
				&Ok(&context),
				&Result::<UserAuthRequestContext, UserActionError>::from(context.clone())
					.map(|ctx| ctx.into())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestActionNoAuth::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: UserActionError::Authenticated,
				})
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_no_auth_not_allowed_unconfirmed() {
		run_test(|_| async {
			let context = UserRequestContextBuilder::build_unconfirmed();
			let action_context = ActionContext {
				action_type: TestActionNoAuth::action_type(),
				context: Some(context.clone()),
			};

			assert_eq!(
				&Ok(&context),
				&Result::<UserUnconfirmedRequestContext, UserActionError>::from(context.clone())
					.map(|ctx| ctx.into())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestActionNoAuth::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: UserActionError::Authenticated,
				})
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_no_auth_ok() {
		run_test(|helper| async move {
			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: TestActionNoAuth::action_type(),
				context: Some(context.clone()),
			};

			assert_eq!(
				&Ok(&context),
				&Result::<UserNoAuthRequestContext, UserActionError>::from(context.clone())
					.map(|ctx| ctx.into())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestActionNoAuth::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Ok(UserOutputInfo {
					action_context,
					data: (),
				}),
			);
			assert_eq!(
				&helper.pop_log(),
				&Some("INFO - user action test (no auth)".into())
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_auth_not_allowed_unauthenticated() {
		run_test(|_| async {
			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: TestActionAuth::action_type(),
				context: Some(context.clone()),
			};

			assert_eq!(
				&Ok(&context),
				&Result::<UserNoAuthRequestContext, UserActionError>::from(context.clone())
					.map(|ctx| ctx.into())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestActionAuth::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: UserActionError::Unauthenticated,
				})
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_auth_not_allowed_unconfirmed() {
		run_test(|_| async {
			let context = UserRequestContextBuilder::build_unconfirmed();
			let action_context = ActionContext {
				action_type: TestActionAuth::action_type(),
				context: Some(context.clone()),
			};

			assert_eq!(
				&Ok(&context),
				&Result::<UserUnconfirmedRequestContext, UserActionError>::from(context.clone())
					.map(|ctx| ctx.into())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestActionAuth::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: UserActionError::Unauthenticated,
				})
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_auth_ok() {
		run_test(|helper| async move {
			let context = UserRequestContextBuilder::new()
				.session(UserSession::Auth(
					UserAuthSessionBuilder::new().user_id(3).build(),
				))
				.build();
			let action_context = ActionContext {
				action_type: TestActionAuth::action_type(),
				context: Some(context.clone()),
			};

			assert_eq!(
				&Ok(&context),
				&Result::<UserAuthRequestContext, UserActionError>::from(context.clone())
					.map(|ctx| ctx.into())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestActionAuth::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Ok(UserOutputInfo {
					action_context,
					data: (),
				}),
			);
			assert_eq!(
				&helper.pop_log(),
				&Some("INFO - user action test (auth): 3".into())
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_unconfirmed_not_allowed_unauthenticated() {
		run_test(|_| async {
			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: TestActionUnconfirmed::action_type(),
				context: Some(context.clone()),
			};

			assert_eq!(
				&Ok(&context),
				&Result::<UserNoAuthRequestContext, UserActionError>::from(context.clone())
					.map(|ctx| ctx.into())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestActionUnconfirmed::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: UserActionError::Unauthenticated,
				})
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_unconfirmed_not_allowed_authenticated() {
		run_test(|_| async {
			let context = UserRequestContextBuilder::build_auth();
			let action_context = ActionContext {
				action_type: TestActionUnconfirmed::action_type(),
				context: Some(context.clone()),
			};

			assert_eq!(
				&Ok(&context),
				&Result::<UserAuthRequestContext, UserActionError>::from(context.clone())
					.map(|ctx| ctx.into())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestActionUnconfirmed::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: UserActionError::Authenticated,
				})
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_unconfirmed_ok() {
		run_test(|helper| async move {
			let context = UserRequestContextBuilder::new()
				.session(UserSession::Unconfirmed(
					UserUnconfirmedSessionBuilder::new().user_id(4).build(),
				))
				.build();
			let action_context = ActionContext {
				action_type: TestActionUnconfirmed::action_type(),
				context: Some(context.clone()),
			};

			assert_eq!(
				&Ok(&context),
				&Result::<UserUnconfirmedRequestContext, UserActionError>::from(context.clone())
					.map(|ctx| ctx.into())
					.as_ref(),
				"Test context reversible change"
			);

			let result = TestActionUnconfirmed::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Ok(UserOutputInfo {
					action_context,
					data: (),
				}),
			);
			assert_eq!(
				&helper.pop_log(),
				&Some("INFO - user action test (unconfirmed): 4".into())
			);
		})
		.await;
	}
}
