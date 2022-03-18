use std::fmt::Debug;

use crate::{
	business::action::{
		action_type::moderator_action_type::ModeratorActionType,
		data::{
			action_data::{BusinessException, ErrorData},
			moderator_action_data::{ModeratorRequestContext, ModeratorSession},
		},
		definition::{
			action_helpers::DescriptiveRequestContext,
			business_action::{ModeratorAction, ModeratorActionResult},
		},
	},
	lib::core::action::{Action, ActionScope, ActionType, RequestInput},
};

impl DescriptiveRequestContext for ModeratorRequestContext {
	fn description(&self) -> String {
		let ModeratorRequestContext {
			action_type,
			session: ModeratorSession { user_id, .. },
			..
		} = &self;
		let action_id = action_type.get_id();
		format!("action({action_id}: {action_type:?}), moderator({user_id:?})")
	}
}

impl
	ActionType<
		ModeratorRequestContext,
		Option<ErrorData>,
		BusinessException<ModeratorRequestContext>,
		u32,
	> for ModeratorActionType
{
	fn scope() -> ActionScope {
		ActionScope::Moderator
	}

	fn id(&self) -> u32 {
		self.get_id()
	}

	fn validate(
		&self,
		_: &ModeratorRequestContext,
	) -> Result<(), BusinessException<ModeratorRequestContext>> {
		match self {
			ModeratorActionType::EchoInfo => Ok(()),
			ModeratorActionType::EchoWarn => Ok(()),
			ModeratorActionType::EchoError => Ok(()),
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
		u32,
		ModeratorActionType,
	> for T
where
	I: Debug,
	O: Debug,
	T: ModeratorAction<I, O>,
{
	fn action_type() -> ModeratorActionType {
		Self::action_type()
	}

	fn new(input: RequestInput<I, ModeratorRequestContext>) -> Self {
		Self::new(input)
	}

	fn input(&self) -> &RequestInput<I, ModeratorRequestContext> {
		self.input()
	}

	fn run(self) -> ModeratorActionResult<O> {
		Self::action_type().validate(&self.input().context)?;
		self.run_inner()
	}
}
