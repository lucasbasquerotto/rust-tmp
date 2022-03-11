use crate::action::Identifiable;

pub enum UserAction {
	LOGIN,
}

impl Identifiable<u32> for UserAction {
	fn id(&self) -> u32 {
		match self {
			UserAction::LOGIN => 1,
		}
	}
}
