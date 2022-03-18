#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserActionType {
	Login,
	Logout,
}

impl UserActionType {
	pub fn get_id(&self) -> u32 {
		match self {
			UserActionType::Login => 1,
			UserActionType::Logout => 2,
		}
	}
}
