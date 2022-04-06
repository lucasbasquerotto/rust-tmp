use crate::core::action::{
	action_type::{general_action_type::ActionType, moderator_action_type::ModeratorActionType},
	data::{
		action_data::{ActionContext, DescriptiveError, ErrorData, RequestInput},
		moderator_action_data::{ModeratorActionError, ModeratorRequestContext, ModeratorSession},
	},
};
use crate::core::action::{
	data::action_data::ActionErrorInfo,
	definition::{
		action::{Action, ActionError, ActionInput, ActionOutput, ModeratorAction},
		action_helpers::DescriptiveRequestContext,
	},
};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

impl<I: ActionInput> ActionInput for RequestInput<I, ModeratorRequestContext> {}

impl DescriptiveRequestContext for ModeratorRequestContext {
	fn description(&self) -> String {
		let ModeratorRequestContext {
			session: ModeratorSession { user_id, .. },
			..
		} = &self;
		format!("moderator({user_id:?})")
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
			ModeratorActionError::NotAllowed(action_type) => Self::error_msg(format!(
				"You are not allowed to execute this action ({action_id}).",
				action_id = action_type.id()
			)),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

impl<I, O, E, T>
	Action<
		RequestInput<I, ModeratorRequestContext>,
		O,
		ActionErrorInfo<ModeratorActionType, ModeratorRequestContext, E>,
	> for T
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError,
	T: ModeratorAction<I, O, E>,
	Self: Sized,
{
	fn run(
		input: RequestInput<I, ModeratorRequestContext>,
	) -> Result<O, ActionErrorInfo<ModeratorActionType, ModeratorRequestContext, E>> {
		let context = input.context.clone();
		let action_type = Self::action_type();
		let allowed =
			context.session.admin || context.session.allowed_actions.contains(&action_type);

		let action_result = if allowed {
			Self::new(Ok(input))
		} else {
			Self::new(Err(ModeratorActionError::NotAllowed(action_type)))
		};

		action_result
			.and_then(|action| action.run_inner())
			.map_err(|error| ActionErrorInfo {
				action_context: ActionContext {
					action_type: Self::action_type(),
					context: context.clone(),
				},
				error,
			})
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use crate::core::action::data::moderator_action_data::tests::{
		moderator_context, ModeratorTestOptions,
	};
	use crate::core::action::data::moderator_action_data::ModeratorActionError;
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
	use crate::tests::test_utils::tests::run_test;

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
			let context = moderator_context(ModeratorTestOptions {
				admin: false,
				allowed_actions: vec![],
			});

			let result = TestAction::run(RequestInput {
				data: (),
				context: context.clone(),
			});
			assert_eq!(
				result,
				Err(ActionErrorInfo {
					action_context: ActionContext {
						action_type: TestAction::action_type(),
						context: context.clone(),
					},
					error: ModeratorActionError::NotAllowed(ModeratorActionType::Test),
				})
			);
		});
	}

	#[test]
	fn test_ok() {
		run_test(|helper| {
			let context = moderator_context(ModeratorTestOptions {
				admin: false,
				allowed_actions: vec![TestAction::action_type()],
			});

			let result = TestAction::run(RequestInput { data: (), context });
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
			let context = moderator_context(ModeratorTestOptions {
				admin: true,
				allowed_actions: vec![],
			});

			let result = TestAction::run(RequestInput { data: (), context });
			assert_eq!(result, Ok(()));
			assert_eq!(
				helper.pop_log(),
				Some("INFO - moderator action test".to_string())
			);
		});
	}
}
