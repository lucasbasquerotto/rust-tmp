use crate::{
	business::action::{
		action_type::moderator_action_type::ModeratorActionType,
		data::{
			action_data::{BusinessException, ErrorData},
			moderator_action_data::{ModeratorRequestContext, ModeratorSession},
		},
		definition::{
			action_helpers::DescriptiveRequestContext,
			business_action::{ActionInput, ActionOutput, ModeratorAction, ModeratorActionResult},
		},
	},
	lib::core::action::{Action, RequestInput},
};

impl DescriptiveRequestContext for ModeratorRequestContext {
	fn description(&self) -> String {
		let ModeratorRequestContext {
			action_type,
			session: ModeratorSession { user_id, .. },
			..
		} = &self;
		let action_id = action_type.id();
		format!("action({action_id}: {action_type:?}), moderator({user_id:?})")
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
		//&input.context.action_type == Self::action_type();
		&input
			.context
			.session
			.allowed_actions
			.contains(&Self::action_type().id());

		Self::new(input)
	}

	fn run(self) -> ModeratorActionResult<O> {
		//TODO: Self::action_type().validate(&self.input().context)?;
		self.run_inner()
	}
}
