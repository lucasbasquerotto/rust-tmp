use crate::business::action::data::action_data::ActionScope;

use super::action_type::ActionType;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserActionType {
	Login,
	Logout,
}

impl UserActionType {}

impl ActionType for UserActionType {
	fn scope() -> ActionScope {
		ActionScope::User
	}

	fn id(&self) -> u32 {
		match self {
			UserActionType::Login => 1,
			UserActionType::Logout => 2,
		}
	}
}
