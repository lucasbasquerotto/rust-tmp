use crate::business::{
	action_type::automatic_action_type::AutomaticActionType,
	data::{
		action_data::{ErrorContext, ErrorData, RequestInput},
		automatic_action_data::{
			AutomaticActionError, AutomaticErrorInput, AutomaticRequest, AutomaticRequestContext,
			HookRequestContext, InternalRequestContext,
		},
	},
	definition::action::{ActionInput, ActionOutput},
	definition::action_helpers::DescriptiveRequestContext,
	definition::{
		action::{Action, ActionError, AutomaticAction},
		action_helpers::ActionErrorHelper,
	},
};

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

impl ActionError<AutomaticActionType, AutomaticRequestContext> for AutomaticActionError {
	fn error_context(&self) -> &ErrorContext<AutomaticActionType, AutomaticRequestContext> {
		match self {
			AutomaticActionError::NotInternal(input) => &input.error_context,
			AutomaticActionError::NotHook(input) => &input.error_context,
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			AutomaticActionError::NotInternal(_) => {
				self.error_msg("This is not an internal action.".to_string())
			}
			AutomaticActionError::NotHook(_) => {
				self.error_msg("This is not a hook action.".to_string())
			}
		}
	}
}

impl AutomaticRequestContext {
	#[allow(dead_code)]
	pub fn to_internal(
		&self,
		action_type: AutomaticActionType,
	) -> Result<InternalRequestContext, AutomaticActionError> {
		let AutomaticRequestContext {
			application,
			request,
		} = self.clone();

		match request {
			AutomaticRequest::Internal => Ok(InternalRequestContext { application }),
			_ => Err(AutomaticActionError::NotInternal(AutomaticErrorInput {
				error_context: ErrorContext {
					action_type,
					context: self.clone(),
				},
				data: (),
			})),
		}
	}

	#[allow(dead_code)]
	pub fn to_hook(
		&self,
		action_type: AutomaticActionType,
	) -> Result<HookRequestContext, AutomaticActionError> {
		let AutomaticRequestContext {
			application,
			request,
		} = self.clone();

		match request {
			AutomaticRequest::Hook(hook_request) => Ok(HookRequestContext {
				application,
				request: hook_request,
			}),
			_ => Err(AutomaticActionError::NotHook(AutomaticErrorInput {
				error_context: ErrorContext {
					action_type,
					context: self.clone(),
				},
				data: (),
			})),
		}
	}
}

impl<I> RequestInput<I, AutomaticRequestContext> {
	#[allow(dead_code)]
	pub fn to_internal(
		self,
		action_type: AutomaticActionType,
	) -> Result<RequestInput<I, InternalRequestContext>, AutomaticActionError> {
		let context = self.context.to_internal(action_type)?;
		Ok(RequestInput {
			context,
			data: self.data,
		})
	}

	#[allow(dead_code)]
	pub fn to_hook(
		self,
		action_type: AutomaticActionType,
	) -> Result<RequestInput<I, HookRequestContext>, AutomaticActionError> {
		let context = self.context.to_hook(action_type)?;
		Ok(RequestInput {
			context,
			data: self.data,
		})
	}
}

impl InternalRequestContext {
	pub fn to_general(&self) -> AutomaticRequestContext {
		let InternalRequestContext { application } = self.clone();

		AutomaticRequestContext {
			application,
			request: AutomaticRequest::Internal,
		}
	}
}

impl<T> RequestInput<T, InternalRequestContext> {
	#[allow(dead_code)]
	pub fn to_general(self) -> RequestInput<T, AutomaticRequestContext> {
		let context = self.context.to_general();
		RequestInput {
			context,
			data: self.data,
		}
	}
}

impl HookRequestContext {
	pub fn to_general(&self) -> AutomaticRequestContext {
		let HookRequestContext {
			application,
			request,
			..
		} = self.clone();

		AutomaticRequestContext {
			application,
			request: AutomaticRequest::Hook(request),
		}
	}
}

impl<T> RequestInput<T, HookRequestContext> {
	#[allow(dead_code)]
	pub fn to_general(self) -> RequestInput<T, AutomaticRequestContext> {
		let context = self.context.to_general();
		RequestInput {
			context,
			data: self.data,
		}
	}
}

impl<I: ActionInput> ActionInput for RequestInput<I, AutomaticRequestContext> {}

impl<I, O, E, T> Action<RequestInput<I, AutomaticRequestContext>, O, E> for T
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError<AutomaticActionType, AutomaticRequestContext>,
	T: AutomaticAction<I, O, E>,
{
	fn run(input: RequestInput<I, AutomaticRequestContext>) -> Result<O, E> {
		let action = Self::new(Ok(input))?;
		action.run_inner()
	}
}

#[cfg(test)]
pub mod tests {
	use crate::business::action_type::automatic_action_type::AutomaticActionType;
	use crate::business::data::action_data::{ErrorContext, ErrorInput};
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
					let real_input = ok_input.to_hook(Self::action_type());

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
					let real_input = ok_input.to_internal(Self::action_type());

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
				input
					.to_internal(TestAction::action_type())
					.map(|ctx| ctx.to_general().context),
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
				input
					.to_hook(TestAction::action_type())
					.map(|ctx| ctx.to_general().context),
				"Test input context reversible change"
			);
		});
	}

	#[test]
	fn test_ok_hook() {
		run_test(|helper| {
			let context = automatic_context(AutomaticTestOptions { internal: false });

			let result = TestAction::run(RequestInput {
				data: (),
				context: context.clone(),
			});
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

			let result = TestAction::run(RequestInput {
				data: (),
				context: context.clone(),
			});
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

			let result = TestActionHook::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(
				result,
				Err(AutomaticActionError::NotHook(ErrorInput {
					error_context: ErrorContext {
						action_type: AutomaticActionType::Test,
						context: context.clone()
					},
					data: ()
				}))
			);
		});
	}

	#[test]
	fn test_hook_ok() {
		run_test(|helper| {
			let context = automatic_context(AutomaticTestOptions { internal: false });

			let result = TestActionHook::run(RequestInput {
				data: (),
				context: context.clone(),
			});
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

			let result = TestActionInternal::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(
				result,
				Err(AutomaticActionError::NotInternal(ErrorInput {
					error_context: ErrorContext {
						action_type: AutomaticActionType::Test,
						context: context.clone()
					},
					data: ()
				}))
			);
		});
	}

	#[test]
	fn test_internal_ok() {
		run_test(|helper| {
			let context = automatic_context(AutomaticTestOptions { internal: true });

			let result = TestActionInternal::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("INFO - automatic action test (only internal)".to_string())
			);
		});
	}
}
