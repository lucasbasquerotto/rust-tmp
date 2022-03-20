use super::action_data::{Application, Request, Session};

#[derive(Clone, Debug)]
pub struct UserSession {
	pub user_id: Option<u64>,
}

impl Session for UserSession {}

#[derive(Clone, Debug)]
pub struct UserAuthSession {
	pub user_id: u64,
}

impl Session for UserAuthSession {}

#[derive(Clone, Debug)]
pub struct UserNoAuthSession();

impl Session for UserNoAuthSession {}

#[derive(Clone, Debug)]
pub struct UserRequestContext {
	pub application: Application,
	pub session: UserSession,
	pub request: Request,
}

#[derive(Clone, Debug)]
pub struct UserAuthRequestContext {
	pub application: Application,
	pub session: UserAuthSession,
	pub request: Request,
}

#[derive(Clone, Debug)]
pub struct UserNoAuthRequestContext {
	pub application: Application,
	pub session: UserNoAuthSession,
	pub request: Request,
}
