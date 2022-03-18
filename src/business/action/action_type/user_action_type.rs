use crate::{
	business::action::{
		data::user_action_data::UserRequestContext, definition::business_action::BusinessActionType,
	},
	lib::core::action::ActionScope,
};

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
