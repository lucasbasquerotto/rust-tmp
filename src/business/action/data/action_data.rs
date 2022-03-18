use std::{collections::HashMap, fmt::Debug};

use crate::lib::core::action::RequestContext;

#[derive(Clone, Debug)]
pub struct Request {
	pub ip: String,
}

pub trait Session: Clone + Debug {}

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
	pub msg: String,
	pub params: Option<HashMap<String, String>>,
	pub meta: Option<HashMap<String, String>>,
}

pub type ActionRequestResult<T> = Result<T, Option<ErrorData>>;
