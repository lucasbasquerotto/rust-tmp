use std::borrow::Cow;

use crate::core::action::{
	data::{
		action_data::{ActionContext, DescriptiveError, ErrorData, RequestInput},
		automatic_action_data::{
			AutomaticActionError, AutomaticErrorInfo, AutomaticOutputInfo, AutomaticRequest,
			AutomaticRequestContext, AutomaticRequestInput, HookRequestContext,
			InternalRequestContext,
		},
	},
	definition::{action::ActionResult, action_helpers::DescriptiveInfo},
};
use crate::core::action::{
	definition::action::{Action, ActionError, AutomaticAction},
	definition::action::{ActionInput, ActionOutput},
};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

impl<I: ActionInput> ActionInput for RequestInput<I, AutomaticRequestContext> {}

impl DescriptiveInfo for AutomaticRequest {
	fn description(&self) -> Cow<'_, str> {
		match self {
			AutomaticRequest::Internal => "automatic(internal)".into(),
			AutomaticRequest::Hook(_) => "automatic(hook)".into(),
		}
	}
}

impl DescriptiveInfo for AutomaticRequestContext {
	fn description(&self) -> Cow<'_, str> {
		let AutomaticRequestContext { request, .. } = &self;
		request.description()
	}
}

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

impl From<AutomaticRequestContext> for Result<InternalRequestContext, AutomaticActionError> {
	fn from(from: AutomaticRequestContext) -> Self {
		let AutomaticRequestContext {
			application,
			request,
		} = from;

		match request {
			AutomaticRequest::Internal => Ok(InternalRequestContext { application }),
			_ => Err(AutomaticActionError::NotInternal),
		}
	}
}

impl From<InternalRequestContext> for AutomaticRequestContext {
	fn from(from: InternalRequestContext) -> Self {
		let InternalRequestContext { application } = from;
		Self {
			application,
			request: AutomaticRequest::Internal,
		}
	}
}

impl<I> From<RequestInput<I, AutomaticRequestContext>>
	for Result<RequestInput<I, InternalRequestContext>, AutomaticActionError>
{
	fn from(from: RequestInput<I, AutomaticRequestContext>) -> Self {
		let context: Result<InternalRequestContext, AutomaticActionError> = from.context.into();
		let context = context?;
		Ok(RequestInput {
			context,
			data: from.data,
		})
	}
}

impl<T> From<RequestInput<T, InternalRequestContext>> for RequestInput<T, AutomaticRequestContext> {
	fn from(from: RequestInput<T, InternalRequestContext>) -> Self {
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

impl From<AutomaticRequestContext> for Result<HookRequestContext, AutomaticActionError> {
	fn from(from: AutomaticRequestContext) -> Self {
		let AutomaticRequestContext {
			application,
			request,
		} = from;

		match request {
			AutomaticRequest::Hook(hook_request) => Ok(HookRequestContext {
				application,
				request: hook_request,
			}),
			_ => Err(AutomaticActionError::NotHook),
		}
	}
}

impl From<HookRequestContext> for AutomaticRequestContext {
	fn from(from: HookRequestContext) -> Self {
		let HookRequestContext {
			application,
			request,
			..
		} = from;

		Self {
			application,
			request: AutomaticRequest::Hook(request),
		}
	}
}

impl<I> From<RequestInput<I, AutomaticRequestContext>>
	for Result<RequestInput<I, HookRequestContext>, AutomaticActionError>
{
	fn from(from: RequestInput<I, AutomaticRequestContext>) -> Self {
		let context: Result<HookRequestContext, AutomaticActionError> = from.context.into();
		let context = context?;
		Ok(RequestInput {
			context,
			data: from.data,
		})
	}
}

impl<T> From<RequestInput<T, HookRequestContext>> for RequestInput<T, AutomaticRequestContext> {
	fn from(from: RequestInput<T, HookRequestContext>) -> Self {
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

impl ActionError for AutomaticActionError {
	fn private_error(&self) -> DescriptiveError {
		match self {
			AutomaticActionError::NotInternal => DescriptiveError::empty(),
			AutomaticActionError::NotHook => DescriptiveError::empty(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			AutomaticActionError::NotInternal => {
				Self::error_msg("This is not an internal action.".into())
			}
			AutomaticActionError::NotHook => Self::error_msg("This is not a hook action.".into()),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

impl<I: 'static, O, E, T>
	Action<AutomaticRequestInput<I>, AutomaticOutputInfo<O>, AutomaticErrorInfo<E>> for T
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError,
	T: AutomaticAction<I, O, E>,
{
	fn run(
		input: AutomaticRequestInput<I>,
	) -> ActionResult<AutomaticOutputInfo<O>, AutomaticErrorInfo<E>> {
		Box::pin(async {
			let action_context = ActionContext {
				action_type: Self::action_type(),
				context: input.context.clone(),
			};

			let action_result = Self::new(Box::pin(async { Ok(input) })).await;

			match action_result {
				Ok(action) => {
					let result = action.run_inner().await;

					match result {
						Ok(data) => Ok(AutomaticOutputInfo {
							action_context,
							data,
						}),
						Err(error) => Err(AutomaticErrorInfo {
							action_context,
							error,
						}),
					}
				}
				Err(error) => Err(AutomaticErrorInfo {
					action_context,
					error,
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
	use crate::core::action::data::automatic_action_data::tests::AutomaticRequestContextBuilder;
	use crate::core::action::data::automatic_action_data::AutomaticOutputInfo;
	use crate::core::action::data::automatic_action_data::{
		AutomaticActionError, AutomaticRequest, HookRequestContext, InternalRequestContext,
	};
	use crate::core::action::data::{
		action_data::RequestInput, automatic_action_data::AutomaticRequestContext,
	};
	use crate::core::action::definition::action::AutomaticAction;
	use crate::core::action::definition::action::{Action, ActionResult};
	use crate::core::action::{
		action_type::automatic_action_type::AutomaticActionType,
		data::action_data::{ActionContext, ActionErrorInfo},
	};
	use crate::tests::test_utils::tests::run_test;

	#[derive(Debug)]
	pub struct TestAction(RequestInput<(), AutomaticRequestContext>);

	#[derive(Debug)]
	pub struct TestActionHook(RequestInput<(), HookRequestContext>);

	#[derive(Debug)]
	pub struct TestActionInternal(RequestInput<(), InternalRequestContext>);

	impl AutomaticAction<(), (), AutomaticActionError> for TestAction {
		fn action_type() -> AutomaticActionType {
			AutomaticActionType::Test
		}

		fn new(
			input: ActionResult<RequestInput<(), AutomaticRequestContext>, AutomaticActionError>,
		) -> ActionResult<Self, AutomaticActionError> {
			Box::pin(async {
				let ok_input = input.await?;
				Ok(Self(ok_input))
			})
		}

		fn run_inner(self) -> ActionResult<(), AutomaticActionError> {
			Box::pin(async move {
				match self.0.context.request {
					AutomaticRequest::Internal => info!("automatic action test (internal)"),
					AutomaticRequest::Hook(_) => info!("automatic action test (hook)"),
				};
				Ok(())
			})
		}
	}

	impl AutomaticAction<(), (), AutomaticActionError> for TestActionHook {
		fn action_type() -> AutomaticActionType {
			AutomaticActionType::Test
		}

		fn new(
			input: ActionResult<RequestInput<(), AutomaticRequestContext>, AutomaticActionError>,
		) -> ActionResult<Self, AutomaticActionError> {
			Box::pin(async {
				match input.await {
					Err(err) => Err(err),
					Ok(ok_input) => {
						let real_input = ok_input.into();

						match real_input {
							Err(err) => Err(err),
							Ok(real_ok_input) => Ok(Self(real_ok_input)),
						}
					}
				}
			})
		}

		fn run_inner(self) -> ActionResult<(), AutomaticActionError> {
			Box::pin(async {
				info!("automatic action test (only hook)");
				Ok(())
			})
		}
	}

	impl AutomaticAction<(), (), AutomaticActionError> for TestActionInternal {
		fn action_type() -> AutomaticActionType {
			AutomaticActionType::Test
		}

		fn new(
			input: ActionResult<RequestInput<(), AutomaticRequestContext>, AutomaticActionError>,
		) -> ActionResult<Self, AutomaticActionError> {
			Box::pin(async {
				match input.await {
					Err(err) => Err(err),
					Ok(ok_input) => {
						let real_input = ok_input.into();

						match real_input {
							Err(err) => Err(err),
							Ok(real_ok_input) => Ok(Self(real_ok_input)),
						}
					}
				}
			})
		}

		fn run_inner(self) -> ActionResult<(), AutomaticActionError> {
			Box::pin(async {
				info!("automatic action test (only internal)");
				Ok(())
			})
		}
	}

	#[tokio::test]
	async fn test_input_context_internal() {
		run_test(|_| async {
			let context = AutomaticRequestContextBuilder::build_internal();
			let input = RequestInput { context, data: () };
			assert_eq!(
				&Ok(input.context.clone()),
				&Result::<RequestInput<(), InternalRequestContext>, AutomaticActionError>::from(
					input
				)
				.map(|ctx| RequestInput::<(), AutomaticRequestContext>::from(ctx).context),
				"Test input context reversible change"
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_input_context_hook() {
		run_test(|_| async {
			let context = AutomaticRequestContextBuilder::build_hook();
			let input = RequestInput { context, data: () };
			assert_eq!(
				&Ok(input.context.clone()),
				&Result::<RequestInput<(), HookRequestContext>, AutomaticActionError>::from(input)
					.map(|ctx| RequestInput::<(), AutomaticRequestContext>::from(ctx).context),
				"Test input context reversible change"
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_ok_hook() {
		run_test(|helper| async move {
			let context = AutomaticRequestContextBuilder::build_hook();
			let action_context = ActionContext {
				action_type: TestAction::action_type(),
				context: context.clone(),
			};

			let result = TestAction::run(RequestInput { data: (), context }).await;
			assert_eq!(
				&result,
				&Ok(AutomaticOutputInfo {
					action_context,
					data: (),
				})
			);
			assert_eq!(
				&helper.pop_log(),
				&Some("INFO - automatic action test (hook)".into())
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_ok_internal() {
		run_test(|helper| async move {
			let context = AutomaticRequestContextBuilder::build_internal();
			let action_context = ActionContext {
				action_type: TestAction::action_type(),
				context: context.clone(),
			};

			let result = TestAction::run(RequestInput { data: (), context }).await;
			assert_eq!(
				&result,
				&Ok(AutomaticOutputInfo {
					action_context,
					data: (),
				})
			);
			assert_eq!(
				&helper.pop_log(),
				&Some("INFO - automatic action test (internal)".into())
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_hook_not_allowed() {
		run_test(|_| async {
			let context = AutomaticRequestContextBuilder::build_internal();
			let action_context = ActionContext {
				action_type: TestActionHook::action_type(),
				context: context.clone(),
			};

			let result = TestActionHook::run(RequestInput { data: (), context }).await;
			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: AutomaticActionError::NotHook,
				})
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_hook_ok() {
		run_test(|helper| async move {
			let context = AutomaticRequestContextBuilder::build_hook();
			let action_context = ActionContext {
				action_type: TestActionHook::action_type(),
				context: context.clone(),
			};

			let result = TestActionHook::run(RequestInput { data: (), context }).await;
			assert_eq!(
				&result,
				&Ok(AutomaticOutputInfo {
					action_context,
					data: (),
				})
			);
			assert_eq!(
				&helper.pop_log(),
				&Some("INFO - automatic action test (only hook)".into())
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_internal_not_allowed() {
		run_test(|_| async {
			let context = AutomaticRequestContextBuilder::build_hook();
			let action_context = ActionContext {
				action_type: TestActionInternal::action_type(),
				context: context.clone(),
			};

			let result = TestActionInternal::run(RequestInput { data: (), context }).await;
			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: AutomaticActionError::NotInternal,
				})
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_internal_ok() {
		run_test(|helper| async move {
			let context = AutomaticRequestContextBuilder::build_internal();
			let action_context = ActionContext {
				action_type: TestActionInternal::action_type(),
				context: context.clone(),
			};

			let result = TestActionInternal::run(RequestInput { data: (), context }).await;
			assert_eq!(
				&result,
				&Ok(AutomaticOutputInfo {
					action_context,
					data: (),
				})
			);
			assert_eq!(
				&helper.pop_log(),
				&Some("INFO - automatic action test (only internal)".into())
			);
		})
		.await;
	}
}