use crate::{
	business::action::{
		data::moderator_action_data::ModeratorRequestContext,
		definition::business_action::BusinessActionType,
	},
	lib::core::action::ActionScope,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModeratorActionType {
	EchoInfo,
	EchoWarn,
	EchoError,
}

impl ModeratorActionType {
	pub fn id(&self) -> u32 {
		match self {
			ModeratorActionType::EchoInfo => 1,
			ModeratorActionType::EchoWarn => 2,
			ModeratorActionType::EchoError => 3,
		}
	}
}

impl BusinessActionType<ModeratorRequestContext> for ModeratorActionType {
	fn scope() -> ActionScope {
		ActionScope::Moderator
	}
}
