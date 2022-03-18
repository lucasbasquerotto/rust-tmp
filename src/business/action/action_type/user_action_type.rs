use crate::{
	business::action::data::user_action_data::UserRequestContext, lib::core::action::ActionScope,
};

use super::action_type::{BusinessActionType, BusinessRequestContext};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserActionType {
	Login,
	Logout,
}

impl UserActionType {
	pub fn id(&self) -> u32 {
		match self {
			UserActionType::Login => 1,
			UserActionType::Logout => 2,
		}
	}
}

impl BusinessActionType<UserRequestContext> for UserActionType {
	fn scope() -> ActionScope {
		ActionScope::User
	}
}

impl BusinessRequestContext<UserActionType> for UserRequestContext {
	fn action_type(&self) -> &UserActionType {
		&self.action_type
	}
}
