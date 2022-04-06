use crate::core::action::{
	action_type::moderator_action_type::ModeratorActionType,
	data::action_data::{ActionErrorInfo, ActionResultInfo, RequestInput},
};

use super::action_data::{Application, Request, Session};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

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

pub type ModeratorRequestInput<I> = RequestInput<I, ModeratorRequestContext>;

pub type ModeratorActionInput<I> = Result<ModeratorRequestInput<I>, ModeratorActionError>;

pub type ModeratorOutputInfo<D> = ActionResultInfo<ModeratorActionType, ModeratorRequestContext, D>;

pub type ModeratorErrorInfo<E> = ActionErrorInfo<ModeratorActionType, ModeratorRequestContext, E>;

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq)]
pub enum ModeratorActionError {
	NotAllowed(ModeratorActionType),
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use crate::core::action::action_type::moderator_action_type::ModeratorActionType;
	use crate::core::action::data::action_data::{Application, Request};

	use super::{ModeratorRequestContext, ModeratorSession};

	#[derive(Debug, Clone)]
	pub struct ModeratorTestOptions {
		pub admin: bool,
		pub allowed_actions: Vec<ModeratorActionType>,
	}

	pub fn moderator_context(options: ModeratorTestOptions) -> ModeratorRequestContext {
		ModeratorRequestContext {
			application: Application {
				request_timeout: 1000,
			},
			session: ModeratorSession {
				user_id: 123,
				admin: options.admin,
				allowed_actions: options.allowed_actions,
			},
			request: Request {
				ip: "5.6.7.8".to_string(),
			},
		}
	}
}
