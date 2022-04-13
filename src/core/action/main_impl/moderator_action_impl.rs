use crate::{
	core::action::{
		action_type::general_action_type::ActionType,
		data::{
			action_data::{ActionContext, DescriptiveError, ErrorData, RequestInput},
			moderator_action_data::{
				ModeratorActionError, ModeratorErrorInfo, ModeratorOutputInfo,
				ModeratorRequestContext, ModeratorSession,
			},
		},
	},
	lib::data::result::AsyncResult,
};
use crate::{
	core::action::{
		data::moderator_action_data::ModeratorActionInput,
		definition::{
			action::{Action, ActionError, ActionInput, ActionOutput, ModeratorAction},
			action_helpers::DescriptiveInfo,
		},
	},
	lib::data::str::Str,
};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

impl<I: ActionInput> ActionInput for RequestInput<I, ModeratorRequestContext> {}

impl DescriptiveInfo for ModeratorRequestContext {
	fn description(&self) -> Str {
		let ModeratorRequestContext {
			session: ModeratorSession { user_id, .. },
			..
		} = &self;
		format!("moderator({user_id:?})").into()
	}
}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

impl ActionError for ModeratorActionError {
	fn private_error(&self) -> DescriptiveError {
		match self {
			ModeratorActionError::NotAllowed(data) => DescriptiveError::data(data),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			ModeratorActionError::NotAllowed(action_type) => Self::error_msg(
				format!(
					"You are not allowed to execute this action ({action_id}).",
					action_id = action_type.id()
				)
				.into(),
			),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

impl<I: 'static, O, E, T>
	Action<ModeratorActionInput<I>, ModeratorOutputInfo<O>, ModeratorErrorInfo<E>> for T
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError + From<ModeratorActionError>,
	T: ModeratorAction<I, O, E>,
	Self: Sized,
{
	fn run(
		input: ModeratorActionInput<I>,
	) -> AsyncResult<ModeratorOutputInfo<O>, ModeratorErrorInfo<E>> {
		Box::pin(async {
			let context = input
				.as_ref()
				.map(|ok_input| Some(ok_input.context.clone()))
				.unwrap_or(None);
			let action_context = ActionContext {
				action_type: Self::action_type(),
				context: context.clone(),
			};
			let action_type = Self::action_type();

			match input {
				Ok(ok_input) => {
					let allowed = ok_input.context.session.admin
						|| ok_input
							.context
							.session
							.allowed_actions
							.contains(&action_type);

					if !allowed {
						Err(ModeratorErrorInfo {
							action_context,
							error: E::from(ModeratorActionError::NotAllowed(action_type)),
						})
					} else {
						let action_result = Self::new(ok_input).await;

						match action_result {
							Ok(action) => {
								let result = action.run_inner().await;

								match result {
									Ok(data) => Ok(ModeratorOutputInfo {
										action_context,
										data,
									}),
									Err(error) => Err(ModeratorErrorInfo {
										action_context,
										error,
									}),
								}
							}
							Err(error) => Err(ModeratorErrorInfo {
								action_context,
								error,
							}),
						}
					}
				}
				Err(error) => Err(ModeratorErrorInfo {
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
	use crate::core::action::data::moderator_action_data::tests::ModeratorRequestContextBuilder;
	use crate::core::action::data::moderator_action_data::tests::ModeratorSessionBuilder;
	use crate::core::action::data::moderator_action_data::ModeratorActionError;
	use crate::core::action::data::moderator_action_data::ModeratorOutputInfo;
	use crate::core::action::data::{
		action_data::{RequestContext, RequestInput},
		moderator_action_data::ModeratorRequestContext,
	};
	use crate::core::action::definition::action::Action;
	use crate::core::action::definition::action::ModeratorAction;
	use crate::core::action::{
		action_type::moderator_action_type::ModeratorActionType,
		data::action_data::{ActionContext, ActionErrorInfo},
	};
	use crate::lib::data::result::AsyncResult;
	use crate::tests::test_utils::tests::run_test;

	#[derive(Debug)]
	pub struct TestAction<T: RequestContext>(RequestInput<(), T>);

	impl ModeratorAction<(), (), ModeratorActionError> for TestAction<ModeratorRequestContext> {
		fn action_type() -> ModeratorActionType {
			ModeratorActionType::Test
		}

		fn new(
			input: RequestInput<(), ModeratorRequestContext>,
		) -> AsyncResult<Self, ModeratorActionError> {
			Box::pin(async { Ok(Self(input)) })
		}

		fn run_inner(self) -> AsyncResult<(), ModeratorActionError> {
			Box::pin(async {
				info!("moderator action test");
				Ok(())
			})
		}
	}

	fn moderator_context() -> ModeratorRequestContext {
		ModeratorRequestContextBuilder::new()
			.session(
				ModeratorSessionBuilder::new()
					.allowed_actions(vec![TestAction::action_type()])
					.build(),
			)
			.build()
	}

	#[tokio::test]
	async fn test_not_allowed() {
		run_test(|_| async {
			let context = ModeratorRequestContextBuilder::new().build();
			let action_context = ActionContext {
				action_type: TestAction::action_type(),
				context: Some(context.clone()),
			};

			let result = TestAction::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: ModeratorActionError::NotAllowed(ModeratorActionType::Test),
				})
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_ok() {
		run_test(|helper| async move {
			let context = moderator_context();
			let action_context = ActionContext {
				action_type: TestAction::action_type(),
				context: Some(context.clone()),
			};

			let result = TestAction::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Ok(ModeratorOutputInfo {
					action_context,
					data: ()
				}),
			);
			assert_eq!(
				&helper.pop_log(),
				&Some("INFO - moderator action test".into())
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_ok_admin() {
		run_test(|helper| async move {
			let context = ModeratorRequestContextBuilder::build_admin();
			let action_context = ActionContext {
				action_type: TestAction::action_type(),
				context: Some(context.clone()),
			};

			let result = TestAction::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Ok(ModeratorOutputInfo {
					action_context,
					data: ()
				}),
			);
			assert_eq!(
				&helper.pop_log(),
				&Some("INFO - moderator action test".into())
			);
		})
		.await;
	}
}
