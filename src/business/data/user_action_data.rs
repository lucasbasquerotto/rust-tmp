use crate::business::action_type::user_action_type::UserActionType;

use super::action_data::{Application, ErrorInput, Request, Session};

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

pub type UserErrorInput<T> = ErrorInput<UserActionType, UserRequestContext, T>;

#[derive(Debug, Eq, PartialEq)]
pub enum UserActionError {
	Authenticated(UserErrorInput<()>),
	Unauthenticated(UserErrorInput<()>),
}
