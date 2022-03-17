use super::general_action_data::{Application, Request, Session};

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

#[derive(Clone, Debug)]
pub struct ModeratorSession {
	pub user_id: u64,
	pub allowed_actions: Vec<u32>,
}

impl Session for ModeratorSession {}

#[derive(Clone, Debug)]
pub struct ModeratorRequestContext {
	pub application: Application,
	pub session: ModeratorSession,
	pub request: Request,
	pub action_type: ModeratorActionType,
}
