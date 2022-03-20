use crate::lib::core::action::ActionScope;

use super::action_type::BusinessActionType;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModeratorActionType {
	EchoInfo,
	EchoWarn,
	EchoError,
}

impl BusinessActionType for ModeratorActionType {
	fn scope() -> ActionScope {
		ActionScope::Moderator
	}

	fn id(&self) -> u32 {
		match self {
			ModeratorActionType::EchoInfo => 1,
			ModeratorActionType::EchoWarn => 2,
			ModeratorActionType::EchoError => 3,
		}
	}
}
