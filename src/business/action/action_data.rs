use std::collections::HashMap;

use crate::lib::core::action_core::RequestInfo;

#[derive(Clone, Debug)]
pub struct Request {
	pub ip: String,
}

#[derive(Clone, Debug)]
pub struct Session {
	pub user_id: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct ModeratorSession {
	pub user_id: u64,
	pub allowed_actions: u32,
}

#[derive(Clone, Debug)]
pub struct Application {
	pub request_timeout: u32,
}

#[derive(Debug)]
pub struct BusinessException<T: RequestInfo> {
	pub info: Option<T>,
	pub private: Option<ErrorData>,
	pub public: Option<ErrorData>,
}

#[derive(Debug)]
pub struct ErrorData {
	pub key: String,
	pub msg: &'static str,
	pub params: Option<HashMap<String, String>>,
	pub meta: Option<HashMap<String, String>>,
}

pub type ActionRequestResult<T> = Result<T, Option<ErrorData>>;
