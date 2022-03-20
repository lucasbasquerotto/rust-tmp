use crate::{
	business::action::{
		action_type::{
			action_type::BusinessActionType, moderator_action_type::ModeratorActionType,
		},
		data::{
			action_data::{ErrorContext, ErrorData},
			moderator_action_data::{
				ModeratorActionError, ModeratorErrorInput, ModeratorRequestContext,
				ModeratorSession,
			},
		},
		definition::{
			action_error::BusinessException,
			action_helpers::DescriptiveRequestContext,
			business_action::{ActionError, ActionInput, ActionOutput, ModeratorAction},
		},
	},
	lib::core::action::{Action, RequestInput},
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

impl ActionError for ModeratorActionError {}

impl BusinessException<ModeratorActionType, ModeratorRequestContext> for ModeratorActionError {
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

impl<I, O, E, T> Action<ModeratorRequestContext, I, O, E, ModeratorActionType> for T
where
	I: ActionInput,
	O: ActionOutput,
	E: BusinessException<ModeratorActionType, ModeratorRequestContext> + ActionError,
	T: ModeratorAction<I, O, E>,
	Self: Sized,
{
	// fn action_type() -> ModeratorActionType {
	// 	Self::action_type()
	// }

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
