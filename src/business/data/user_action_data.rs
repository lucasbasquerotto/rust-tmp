use super::action_data::{Application, Request, Session};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserSession {
	pub user_id: Option<u64>,
}

impl Session for UserSession {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserAuthSession {
	pub user_id: u64,
}

impl Session for UserAuthSession {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserNoAuthSession();

impl Session for UserNoAuthSession {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserRequestContext {
	pub application: Application,
	pub session: UserSession,
	pub request: Request,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserAuthRequestContext {
	pub application: Application,
	pub session: UserAuthSession,
	pub request: Request,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserNoAuthRequestContext {
	pub application: Application,
	pub session: UserNoAuthSession,
	pub request: Request,
}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq)]
pub enum UserActionError {
	Authenticated,
	Unauthenticated,
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use crate::business::data::action_data::{Application, Request};
	use crate::business::data::user_action_data::{UserRequestContext, UserSession};

	#[derive(Debug, Clone)]
	pub struct UserTestOptions {
		pub user_id: Option<u64>,
	}

	pub fn user_context(options: UserTestOptions) -> UserRequestContext {
		UserRequestContext {
			application: Application {
				request_timeout: 1000,
			},
			session: UserSession {
				user_id: options.user_id,
			},
			request: Request {
				ip: "1.2.3.4".to_string(),
			},
		}
	}
}
