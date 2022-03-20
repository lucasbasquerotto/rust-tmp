use crate::{
	business::action::{
		action_type::{
			action_type::BusinessActionType, moderator_action_type::ModeratorActionType,
		},
		data::{
			action_data::{BusinessException, ErrorData},
			moderator_action_data::{ModeratorRequestContext, ModeratorSession},
		},
		definition::{
			action_error::BusinessErrorGenerator,
			action_helpers::DescriptiveRequestContext,
			business_action::{ActionInput, ActionOutput, ModeratorAction, ModeratorActionResult},
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
		// let action_id = action_type.id();
		// format!("action({action_id}: {action_type:?}), moderator({user_id:?})")
		format!("moderator({user_id:?})")
	}
}

#[derive(Debug)]
enum ModeratorActionError<'a> {
	NotAllowed(&'a u32),
}

impl BusinessErrorGenerator<ModeratorRequestContext> for ModeratorActionError<'_> {
	fn private_error(&self) -> Option<ErrorData> {
		match self {
			ModeratorActionError::NotAllowed(_) => None,
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			ModeratorActionError::NotAllowed(action_id) => self.error_msg(format!(
				"You are not allowed to execute this action ({action_id})."
			)),
		}
	}
}

impl<I, O, T>
	Action<
		ModeratorRequestContext,
		I,
		O,
		Option<ErrorData>,
		BusinessException<ModeratorRequestContext>,
		ModeratorActionType,
	> for T
where
	I: ActionInput,
	O: ActionOutput,
	T: ModeratorAction<I, O>,
	Self: Sized,
{
	fn action_type() -> ModeratorActionType {
		Self::action_type()
	}

	fn new(input: RequestInput<I, ModeratorRequestContext>) -> ModeratorActionResult<Self> {
		let context = &input.context;
		let action_id = &Self::action_type().id();

		if !context.session.allowed_actions.contains(action_id) {
			Err(ModeratorActionError::NotAllowed(action_id).exception(context))?;
		}

		Self::new(input)
	}

	fn run(self) -> ModeratorActionResult<O> {
		self.run_inner()
	}
}
