use crate::business::{
	action_type::{action_type::ActionType, moderator_action_type::ModeratorActionType},
	data::{
		action_data::{ErrorContext, ErrorData, RequestInput},
		moderator_action_data::{
			ModeratorActionError, ModeratorErrorInput, ModeratorRequestContext, ModeratorSession,
		},
	},
	definition::{
		action::{Action, ActionError, ActionInput, ActionOutput, ModeratorAction},
		action_helpers::{ActionErrorHelper, DescriptiveRequestContext},
	},
};

impl DescriptiveRequestContext for ModeratorRequestContext {
	fn description(&self) -> String {
		let ModeratorRequestContext {
			session: ModeratorSession { user_id, .. },
			..
		} = &self;
		format!("moderator({user_id:?})")
	}
}

impl ActionError<ModeratorActionType, ModeratorRequestContext> for ModeratorActionError {
	fn error_context(&self) -> &ErrorContext<ModeratorActionType, ModeratorRequestContext> {
		match self {
			ModeratorActionError::NotAllowed(input) => &input.error_context,
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			ModeratorActionError::NotAllowed(input) => self.error_msg(format!(
				"You are not allowed to execute this action ({action_id}).",
				action_id = input.data
			)),
		}
	}
}

impl<I: ActionInput> ActionInput for RequestInput<I, ModeratorRequestContext> {}

impl<I, O, E, T> Action<RequestInput<I, ModeratorRequestContext>, O, E> for T
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError<ModeratorActionType, ModeratorRequestContext>,
	T: ModeratorAction<I, O, E>,
	Self: Sized,
{
	fn run(input: RequestInput<I, ModeratorRequestContext>) -> Result<O, E> {
		let context = &input.context;
		let action_type = &Self::action_type();
		let allowed =
			context.session.admin || context.session.allowed_actions.contains(action_type);

		let action = if allowed {
			Self::new(Ok(input))?
		} else {
			let error_context = ErrorContext {
				action_type: action_type.clone(),
				context: context.clone(),
			};
			Self::new(Err(ModeratorActionError::NotAllowed(ModeratorErrorInput {
				error_context,
				data: action_type.id(),
			})))?
		};

		action.run_inner()
	}
}

#[cfg(test)]
pub mod tests {
	use crate::business::action_type::moderator_action_type::ModeratorActionType;
	use crate::business::data::action_data::{ErrorContext, ErrorInput};
	use crate::business::data::moderator_action_data::ModeratorActionError;
	use crate::business::definition::action::Action;
	use crate::business::{
		data::{
			action_data::{RequestContext, RequestInput},
			moderator_action_data::ModeratorRequestContext,
		},
		definition::action::ModeratorAction,
	};
	use crate::tests::test_utils::tests::{moderator_context, run_test, ModeratorOptions};
	use business::action_type::action_type::ActionType;

	#[derive(Debug)]
	pub struct TestAction<T: RequestContext>(RequestInput<(), T>);

	impl ModeratorAction<(), (), ModeratorActionError> for TestAction<ModeratorRequestContext> {
		fn action_type() -> ModeratorActionType {
			ModeratorActionType::Test
		}

		fn new(
			input: Result<RequestInput<(), ModeratorRequestContext>, ModeratorActionError>,
		) -> Result<Self, ModeratorActionError> {
			match input {
				Err(err) => Err(err),
				Ok(ok_input) => Ok(Self(ok_input)),
			}
		}

		fn run_inner(self) -> Result<(), ModeratorActionError> {
			info!("moderator action test");
			Ok(())
		}
	}

	#[test]
	fn test_not_allowed() {
		run_test(|_| {
			let context = moderator_context(ModeratorOptions {
				admin: false,
				allowed_actions: vec![],
			});

			let result = TestAction::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(
				result,
				Err(ModeratorActionError::NotAllowed(ErrorInput {
					error_context: ErrorContext {
						action_type: ModeratorActionType::Test,
						context: context.clone()
					},
					data: ModeratorActionType::Test.id()
				}))
			);
		});
	}

	#[test]
	fn test_ok() {
		run_test(|helper| {
			let context = moderator_context(ModeratorOptions {
				admin: false,
				allowed_actions: vec![TestAction::action_type()],
			});

			let result = TestAction::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("INFO - moderator action test".to_string())
			);
		});
	}

	#[test]
	fn test_ok_admin() {
		run_test(|helper| {
			let context = moderator_context(ModeratorOptions {
				admin: true,
				allowed_actions: vec![],
			});

			let result = TestAction::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("INFO - moderator action test".to_string())
			);
		});
	}
}
