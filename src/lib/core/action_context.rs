use std::fmt::Debug;

pub enum ActionContext {
	USER,
	MODERATOR,
	AUTOMATIC,
}

#[derive(Debug)]
pub struct Request {
	pub ip: String,
}

#[derive(Debug)]
pub struct Session {
	pub user_id: u64,
}

#[derive(Debug)]
pub struct ModeratorSession {
	pub user_id: u64,
	pub allowed_actions: u32,
}

#[derive(Debug)]
pub struct Application {
	pub request_timeout: u32,
}

#[derive(Debug)]
pub struct UserInput<T: Debug> {
	pub application: Application,
	pub session: Session,
	pub request: Request,
	pub data: T,
}

#[derive(Debug)]
pub struct ModeratorInput<T: Debug> {
	pub application: Application,
	pub session: ModeratorSession,
	pub request: Request,
	pub data: T,
}

#[derive(Debug)]
pub struct AutomaticInput<T: Debug> {
	pub application: Application,
	pub request: Request,
	pub data: T,
}
