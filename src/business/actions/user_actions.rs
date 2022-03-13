use crate::lib::core::action_core::ActionType;

pub enum UserAction {
	LOGIN,
	LOGOUT,
}

impl ActionType<u32> for UserAction {
	fn id(&self) -> u32 {
		match self {
			UserAction::LOGIN => 1,
			UserAction::LOGOUT => 2,
		}
	}
}
