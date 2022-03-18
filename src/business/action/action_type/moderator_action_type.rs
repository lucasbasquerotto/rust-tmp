#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModeratorActionType {
	EchoInfo,
	EchoWarn,
	EchoError,
}

impl ModeratorActionType {
	pub fn get_id(&self) -> u32 {
		match self {
			ModeratorActionType::EchoInfo => 1,
			ModeratorActionType::EchoWarn => 2,
			ModeratorActionType::EchoError => 3,
		}
	}
}
