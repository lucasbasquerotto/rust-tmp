use crate::business::action_type::moderator_action_type::ModeratorActionType;

use super::action_data::{Application, ErrorInput, Request, Session};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModeratorSession {
	pub user_id: u64,
	pub admin: bool,
	pub allowed_actions: Vec<ModeratorActionType>,
}

impl Session for ModeratorSession {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModeratorRequestContext {
	pub application: Application,
	pub session: ModeratorSession,
	pub request: Request,
}

pub type ModeratorErrorInput<T> = ErrorInput<ModeratorActionType, ModeratorRequestContext, T>;

#[derive(Debug, Eq, PartialEq)]
pub enum ModeratorActionError {
	NotAllowed(ModeratorErrorInput<u32>),
}
