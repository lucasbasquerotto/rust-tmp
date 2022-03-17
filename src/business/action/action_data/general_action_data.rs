use std::{collections::HashMap, fmt::Debug};

use crate::lib::core::action::RequestContext;

#[derive(Clone, Debug)]
pub struct Request {
	pub ip: String,
}

pub trait Session: Clone + Debug {}

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
pub struct ModeratorSession {
	pub user_id: u64,
	pub allowed_actions: Vec<u32>,
}

impl Session for ModeratorSession {}

#[derive(Clone, Debug)]
pub struct Application {
	pub request_timeout: u32,
}

#[derive(Debug)]
pub struct BusinessException<C: RequestContext> {
	pub context: Option<C>,
	pub private: Option<ErrorData>,
	pub public: Option<ErrorData>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorData {
	pub key: String,
	pub msg: &'static str,
	pub params: Option<HashMap<String, String>>,
	pub meta: Option<HashMap<String, String>>,
}

pub type ActionRequestResult<T> = Result<T, Option<ErrorData>>;
