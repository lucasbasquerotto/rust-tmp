use super::action_data::{Application, Request, Session};

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
}
