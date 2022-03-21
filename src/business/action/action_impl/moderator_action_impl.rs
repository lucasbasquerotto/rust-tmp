use crate::business::action::{
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

	fn description(&self) -> String {
		self.default_description()
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
	fn new(input: RequestInput<I, ModeratorRequestContext>) -> Result<Self, E> {
		let context = &input.context;
		let action_type = &Self::action_type();
		let action_id = &action_type.id();

		if !context.session.allowed_actions.contains(action_id) {
			let error_context = ErrorContext {
				action_type: action_type.clone(),
				context: context.clone(),
			};
			Self::new_inner(Err(ModeratorActionError::NotAllowed(ModeratorErrorInput {
				error_context,
				data: action_type.id(),
			})))
		} else {
			Self::new_inner(Ok(input))
		}
	}

	fn run(self) -> Result<O, E> {
		self.run_inner()
	}
}
