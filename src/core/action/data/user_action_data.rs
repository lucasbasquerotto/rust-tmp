use chrono::{DateTime, Utc};

use super::action_data::{Application, Request, Session};
use crate::core::action::{
	action_type::user_action_type::UserActionType,
	data::action_data::{ActionErrorInfo, ActionResultInfo, RequestContext, RequestInput},
};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserSession {
	Auth(UserAuthSession),
	NoAuth(UserNoAuthSession),
	Unconfirmed(UserUnconfirmedSession),
}

impl Session for UserSession {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserAuthSession {
	pub created_at: DateTime<Utc>,
	pub user_id: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserUnconfirmedSession {
	pub created_at: DateTime<Utc>,
	pub user_id: u64,
}

impl Session for UserAuthSession {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserNoAuthSession {
	pub created_at: DateTime<Utc>,
}

impl Session for UserNoAuthSession {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserRequestContext {
	pub application: Application,
	pub session: UserSession,
	pub request: Request,
}

impl RequestContext for UserRequestContext {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserAuthRequestContext {
	pub application: Application,
	pub session: UserAuthSession,
	pub request: Request,
}

impl RequestContext for UserAuthRequestContext {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserNoAuthRequestContext {
	pub application: Application,
	pub session: UserNoAuthSession,
	pub request: Request,
}

impl RequestContext for UserNoAuthRequestContext {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserUnconfirmedRequestContext {
	pub application: Application,
	pub session: UserUnconfirmedSession,
	pub request: Request,
}

impl RequestContext for UserUnconfirmedRequestContext {}

pub type UserRequestInput<I> = RequestInput<I, UserRequestContext>;

#[allow(dead_code)]
pub type UserAuthRequestInput<I> = RequestInput<I, UserAuthRequestContext>;

pub type UserNoAuthRequestInput<I> = RequestInput<I, UserNoAuthRequestContext>;

pub type UserActionInput<I> = Result<UserRequestInput<I>, UserActionError>;

pub type UserOutputInfo<D> = ActionResultInfo<UserActionType, UserRequestContext, D>;

pub type UserErrorInfo<E> = ActionErrorInfo<UserActionType, UserRequestContext, E>;

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
	use chrono::Utc;

	use crate::core::action::data::user_action_data::{
		UserNoAuthSession, UserRequestContext, UserSession,
	};
	use crate::core::action::data::{
		action_data::{Application, Request},
		user_action_data::UserAuthSession,
	};

	#[derive(Debug, Clone)]
	pub struct UserTestOptions {
		pub user_id: Option<u64>,
	}

	pub fn user_context(options: UserTestOptions) -> UserRequestContext {
		UserRequestContext {
			application: Application {
				request_timeout: 1000,
			},
			session: match options.user_id {
				Some(user_id) => UserSession::Auth(UserAuthSession {
					created_at: Utc::now(),
					user_id,
				}),
				None => UserSession::NoAuth(UserNoAuthSession {
					created_at: Utc::now(),
				}),
			},
			request: Request {
				ip: "1.2.3.4".to_string(),
			},
		}
	}
}
