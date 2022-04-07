use crate::core::action::{
	definition::action::{Action, ActionError, AutomaticAction},
	definition::action::{ActionInput, ActionOutput},
};
use crate::{
	core::action::{
		data::{
			action_data::{ActionContext, DescriptiveError, ErrorData, RequestInput},
			automatic_action_data::{
				AutomaticActionError, AutomaticErrorInfo, AutomaticOutputInfo, AutomaticRequest,
				AutomaticRequestContext, AutomaticRequestInput, HookRequestContext,
				InternalRequestContext,
			},
		},
		definition::action_helpers::DescriptiveInfo,
	},
	lib::data::str::Str,
};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

impl<I: ActionInput> ActionInput for RequestInput<I, AutomaticRequestContext> {}

impl DescriptiveInfo for AutomaticRequestContext {
	fn description(&self) -> Str {
		let AutomaticRequestContext { request, .. } = &self;

		match request {
			AutomaticRequest::Internal => "automatic(internal)".into(),
			AutomaticRequest::Hook(_) => "automatic(hook)".into(),
		}
	}
}

impl DescriptiveInfo for InternalRequestContext {
	fn description(&self) -> Str {
		"automatic(internal)".into()
	}
}

impl DescriptiveInfo for HookRequestContext {
	fn description(&self) -> Str {
		"automatic(hook)".into()
	}
}

impl AutomaticRequestContext {
	#[allow(dead_code)]
	pub fn into_internal(self) -> Result<InternalRequestContext, AutomaticActionError> {
		let AutomaticRequestContext {
			application,
			request,
		} = self;

		match request {
			AutomaticRequest::Internal => Ok(InternalRequestContext { application }),
			_ => Err(AutomaticActionError::NotInternal),
		}
	}

	#[allow(dead_code)]
	pub fn into_hook(self) -> Result<HookRequestContext, AutomaticActionError> {
		let AutomaticRequestContext {
			application,
			request,
		} = self;

		match request {
			AutomaticRequest::Hook(hook_request) => Ok(HookRequestContext {
				application,
				request: hook_request,
			}),
			_ => Err(AutomaticActionError::NotHook),
		}
	}
}

impl<I> RequestInput<I, AutomaticRequestContext> {
	#[allow(dead_code)]
	pub fn into_internal(
		self,
	) -> Result<RequestInput<I, InternalRequestContext>, AutomaticActionError> {
		let context = self.context.into_internal()?;
		Ok(RequestInput {
			context,
			data: self.data,
		})
	}

	#[allow(dead_code)]
	pub fn into_hook(self) -> Result<RequestInput<I, HookRequestContext>, AutomaticActionError> {
		let context = self.context.into_hook()?;
		Ok(RequestInput {
			context,
			data: self.data,
		})
	}
}

impl InternalRequestContext {
	pub fn into_general(self) -> AutomaticRequestContext {
		let InternalRequestContext { application } = self;

		AutomaticRequestContext {
			application,
			request: AutomaticRequest::Internal,
		}
	}
}

impl<T> RequestInput<T, InternalRequestContext> {
	#[allow(dead_code)]
	pub fn into_general(self) -> RequestInput<T, AutomaticRequestContext> {
		let context = self.context.into_general();
		RequestInput {
			context,
			data: self.data,
		}
	}
}

impl HookRequestContext {
	pub fn into_general(self) -> AutomaticRequestContext {
		let HookRequestContext {
			application,
			request,
			..
		} = self;

		AutomaticRequestContext {
			application,
			request: AutomaticRequest::Hook(request),
		}
	}
}

impl<T> RequestInput<T, HookRequestContext> {
	#[allow(dead_code)]
	pub fn into_general(self) -> RequestInput<T, AutomaticRequestContext> {
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

impl<I, O, E, T> Action<AutomaticRequestInput<I>, AutomaticOutputInfo<O>, AutomaticErrorInfo<E>>
	for T
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError,
	T: AutomaticAction<I, O, E>,
{
	fn run(
		input: AutomaticRequestInput<I>,
	) -> Result<AutomaticOutputInfo<O>, AutomaticErrorInfo<E>> {
		let action_context = ActionContext {
			action_type: Self::action_type(),
			context: input.context.clone(),
		};

		let result = Self::new(Ok(input)).and_then(|action| action.run_inner());

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
	use crate::core::action::definition::action::Action;
	use crate::core::action::definition::action::AutomaticAction;
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
			input: Result<RequestInput<(), AutomaticRequestContext>, AutomaticActionError>,
		) -> Result<Self, AutomaticActionError> {
			let ok_input = input?;
			Ok(Self(ok_input))
		}

		fn run_inner(self) -> Result<(), AutomaticActionError> {
			match self.0.context.request {
				AutomaticRequest::Internal => info!("automatic action test (internal)"),
				AutomaticRequest::Hook(_) => info!("automatic action test (hook)"),
			};
			Ok(())
		}
	}

	impl AutomaticAction<(), (), AutomaticActionError> for TestActionHook {
		fn action_type() -> AutomaticActionType {
			AutomaticActionType::Test
		}

		fn new(
			input: Result<RequestInput<(), AutomaticRequestContext>, AutomaticActionError>,
		) -> Result<Self, AutomaticActionError> {
			match input {
				Err(err) => Err(err),
				Ok(ok_input) => {
					let real_input = ok_input.into_hook();

					match real_input {
						Err(err) => Err(err),
						Ok(real_ok_input) => Ok(Self(real_ok_input)),
					}
				}
			}
		}

		fn run_inner(self) -> Result<(), AutomaticActionError> {
			info!("automatic action test (only hook)");
			Ok(())
		}
	}

	impl AutomaticAction<(), (), AutomaticActionError> for TestActionInternal {
		fn action_type() -> AutomaticActionType {
			AutomaticActionType::Test
		}

		fn new(
			input: Result<RequestInput<(), AutomaticRequestContext>, AutomaticActionError>,
		) -> Result<Self, AutomaticActionError> {
			match input {
				Err(err) => Err(err),
				Ok(ok_input) => {
					let real_input = ok_input.into_internal();

					match real_input {
						Err(err) => Err(err),
						Ok(real_ok_input) => Ok(Self(real_ok_input)),
					}
				}
			}
		}

		fn run_inner(self) -> Result<(), AutomaticActionError> {
			info!("automatic action test (only internal)");
			Ok(())
		}
	}

	#[test]
	fn test_input_context_internal() {
		run_test(|_| {
			let context = AutomaticRequestContextBuilder::build_internal();
			let input = RequestInput { context, data: () };
			assert_eq!(
				Ok(input.context.clone()),
				input.into_internal().map(|ctx| ctx.into_general().context),
				"Test input context reversible change"
			);
		});
	}

	#[test]
	fn test_input_context_hook() {
		run_test(|_| {
			let context = AutomaticRequestContextBuilder::build_hook();
			let input = RequestInput { context, data: () };
			assert_eq!(
				Ok(input.context.clone()),
				input.into_hook().map(|ctx| ctx.into_general().context),
				"Test input context reversible change"
			);
		});
	}

	#[test]
	fn test_ok_hook() {
		run_test(|helper| {
			let context = AutomaticRequestContextBuilder::build_hook();
			let action_context = ActionContext {
				action_type: TestAction::action_type(),
				context: context.clone(),
			};

			let result = TestAction::run(RequestInput { data: (), context });
			assert_eq!(
				&result,
				&Ok(AutomaticOutputInfo {
					action_context,
					data: (),
				})
			);
			assert_eq!(
				helper.pop_log(),
				Some("INFO - automatic action test (hook)".into())
			);
		});
	}

	#[test]
	fn test_ok_internal() {
		run_test(|helper| {
			let context = AutomaticRequestContextBuilder::build_internal();
			let action_context = ActionContext {
				action_type: TestAction::action_type(),
				context: context.clone(),
			};

			let result = TestAction::run(RequestInput { data: (), context });
			assert_eq!(
				&result,
				&Ok(AutomaticOutputInfo {
					action_context,
					data: (),
				})
			);
			assert_eq!(
				helper.pop_log(),
				Some("INFO - automatic action test (internal)".into())
			);
		});
	}

	#[test]
	fn test_hook_not_allowed() {
		run_test(|_| {
			let context = AutomaticRequestContextBuilder::build_internal();
			let action_context = ActionContext {
				action_type: TestActionHook::action_type(),
				context: context.clone(),
			};

			let result = TestActionHook::run(RequestInput { data: (), context });
			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: AutomaticActionError::NotHook,
				})
			);
		});
	}

	#[test]
	fn test_hook_ok() {
		run_test(|helper| {
			let context = AutomaticRequestContextBuilder::build_hook();
			let action_context = ActionContext {
				action_type: TestActionHook::action_type(),
				context: context.clone(),
			};

			let result = TestActionHook::run(RequestInput { data: (), context });
			assert_eq!(
				&result,
				&Ok(AutomaticOutputInfo {
					action_context,
					data: (),
				})
			);
			assert_eq!(
				helper.pop_log(),
				Some("INFO - automatic action test (only hook)".into())
			);
		});
	}

	#[test]
	fn test_internal_not_allowed() {
		run_test(|_| {
			let context = AutomaticRequestContextBuilder::build_hook();
			let action_context = ActionContext {
				action_type: TestActionInternal::action_type(),
				context: context.clone(),
			};

			let result = TestActionInternal::run(RequestInput { data: (), context });
			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: AutomaticActionError::NotInternal,
				})
			);
		});
	}

	#[test]
	fn test_internal_ok() {
		run_test(|helper| {
			let context = AutomaticRequestContextBuilder::build_internal();
			let action_context = ActionContext {
				action_type: TestActionInternal::action_type(),
				context: context.clone(),
			};

			let result = TestActionInternal::run(RequestInput { data: (), context });
			assert_eq!(
				&result,
				&Ok(AutomaticOutputInfo {
					action_context,
					data: (),
				})
			);
			assert_eq!(
				helper.pop_log(),
				Some("INFO - automatic action test (only internal)".into())
			);
		});
	}
}
