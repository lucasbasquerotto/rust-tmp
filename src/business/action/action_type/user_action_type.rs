use crate::lib::core::action::ActionScope;

use super::action_type::BusinessActionType;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserActionType {
	Login,
	Logout,
}

impl UserActionType {}

impl BusinessActionType for UserActionType {
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
