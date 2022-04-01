use crate::business::{
	data::{
		action_data::{DescriptiveError, ErrorData, RequestInput},
		automatic_action_data::{
			AutomaticActionError, AutomaticRequest, AutomaticRequestContext, HookRequestContext,
			InternalRequestContext,
		},
	},
	definition::action::{Action, ActionError, AutomaticAction},
	definition::action::{ActionInput, ActionOutput},
	definition::action_helpers::DescriptiveRequestContext,
};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

impl<I: ActionInput> ActionInput for RequestInput<I, AutomaticRequestContext> {}

impl DescriptiveRequestContext for AutomaticRequestContext {
	fn description(&self) -> String {
		let AutomaticRequestContext { request, .. } = &self;

		match request {
			AutomaticRequest::Internal => "automatic(internal)".to_string(),
			AutomaticRequest::Hook(_) => "automatic(hook)".to_string(),
		}
	}
}

impl DescriptiveRequestContext for InternalRequestContext {
	fn description(&self) -> String {
		"automatic(internal)".to_string()
	}
}

impl DescriptiveRequestContext for HookRequestContext {
	fn description(&self) -> String {
		"automatic(hook)".to_string()
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
				Self::error_msg("This is not an internal action.".to_string())
			}
			AutomaticActionError::NotHook => {
				Self::error_msg("This is not a hook action.".to_string())
			}
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

impl<I, O, E, T> Action<RequestInput<I, AutomaticRequestContext>, O, E> for T
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError,
	T: AutomaticAction<I, O, E>,
{
	fn run(input: RequestInput<I, AutomaticRequestContext>) -> Result<O, E> {
		let action = Self::new(Ok(input))?;
		action.run_inner()
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use crate::business::action_type::automatic_action_type::AutomaticActionType;
	use crate::business::data::automatic_action_data::tests::{
		automatic_context, AutomaticTestOptions,
	};
	use crate::business::data::automatic_action_data::{
		AutomaticActionError, AutomaticRequest, HookRequestContext, InternalRequestContext,
	};
	use crate::business::definition::action::Action;
	use crate::business::{
		data::{action_data::RequestInput, automatic_action_data::AutomaticRequestContext},
		definition::action::AutomaticAction,
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
			let context = automatic_context(AutomaticTestOptions { internal: true });
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
			let context = automatic_context(AutomaticTestOptions { internal: false });
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
			let context = automatic_context(AutomaticTestOptions { internal: false });

			let result = TestAction::run(RequestInput { data: (), context });
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("INFO - automatic action test (hook)".to_string())
			);
		});
	}

	#[test]
	fn test_ok_internal() {
		run_test(|helper| {
			let context = automatic_context(AutomaticTestOptions { internal: true });

			let result = TestAction::run(RequestInput { data: (), context });
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("INFO - automatic action test (internal)".to_string())
			);
		});
	}

	#[test]
	fn test_hook_not_allowed() {
		run_test(|_| {
			let context = automatic_context(AutomaticTestOptions { internal: true });

			let result = TestActionHook::run(RequestInput { data: (), context });
			assert_eq!(result, Err(AutomaticActionError::NotHook));
		});
	}

	#[test]
	fn test_hook_ok() {
		run_test(|helper| {
			let context = automatic_context(AutomaticTestOptions { internal: false });

			let result = TestActionHook::run(RequestInput { data: (), context });
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("INFO - automatic action test (only hook)".to_string())
			);
		});
	}

	#[test]
	fn test_internal_not_allowed() {
		run_test(|_| {
			let context = automatic_context(AutomaticTestOptions { internal: false });

			let result = TestActionInternal::run(RequestInput { data: (), context });
			assert_eq!(result, Err(AutomaticActionError::NotInternal));
		});
	}

	#[test]
	fn test_internal_ok() {
		run_test(|helper| {
			let context = automatic_context(AutomaticTestOptions { internal: true });

			let result = TestActionInternal::run(RequestInput { data: (), context });
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("INFO - automatic action test (only internal)".to_string())
			);
		});
	}
}
