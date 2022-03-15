// use std::fmt::Debug;

// use super::action_core::RequestInfo;

// #[derive(Debug)]
// pub struct Request {
// 	pub ip: String,
// }

// #[derive(Debug)]
// pub struct Session {
// 	pub user_id: u64,
// }

// #[derive(Debug)]
// pub struct ModeratorSession {
// 	pub user_id: u64,
// 	pub allowed_actions: u32,
// }

// #[derive(Debug)]
// pub struct Application {
// 	pub request_timeout: u32,
// }

// #[derive(Debug)]
// pub struct UserRequestInfo {
// 	pub application: Application,
// 	pub session: Session,
// 	pub request: Request,
// }

// impl RequestInfo for UserRequestInfo {}

// #[derive(Debug)]
// pub struct ModeratorRequestInfo {
// 	pub application: Application,
// 	pub session: ModeratorSession,
// 	pub request: Request,
// }

// impl RequestInfo for ModeratorRequestInfo {}

// #[derive(Debug)]
// pub struct AutomaticRequestInfo {
// 	pub application: Application,
// 	pub request: Request,
// }

// impl RequestInfo for AutomaticRequestInfo {}
