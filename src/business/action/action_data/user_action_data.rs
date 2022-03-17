use super::general_action_data::{
	Application, Request, UserAuthSession, UserNoAuthSession, UserSession,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserActionType {
	LOGIN,
	LOGOUT,
}

#[derive(Clone, Debug)]
pub struct UserRequestContext {
	pub application: Application,
	pub session: UserSession,
	pub request: Request,
	pub action_type: UserActionType,
}

#[derive(Clone, Debug)]
pub struct UserAuthRequestContext {
	pub application: Application,
	pub session: UserAuthSession,
	pub request: Request,
	pub action_type: UserActionType,
}

#[derive(Clone, Debug)]
pub struct UserNoAuthRequestContext {
	pub application: Application,
	pub session: UserNoAuthSession,
	pub request: Request,
	pub action_type: UserActionType,
}
