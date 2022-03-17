use crate::{
	business::action::action_data::{
		general_action_data::{BusinessException, ErrorData},
		moderator_action_data::{ModeratorActionType, ModeratorRequestContext},
	},
	lib::core::action::{ActionScope, ActionType},
};

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
