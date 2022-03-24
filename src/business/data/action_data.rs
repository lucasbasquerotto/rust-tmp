use std::{collections::HashMap, fmt::Debug};

use crate::business::action_type::action_type::ActionType;

pub enum ActionScope {
	User,
	Moderator,
	Automatic,
}

pub trait RequestContext {}

#[derive(Debug)]
pub struct RequestInput<I, C: RequestContext> {
	pub context: C,
	pub data: I,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
	pub ip: String,
}

pub trait Session: Clone + Debug + Eq + PartialEq {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Application {
	pub request_timeout: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorContext<T: ActionType, C: RequestContext> {
	pub action_type: T,
	pub context: C,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorInput<T: ActionType, C: RequestContext, D> {
	pub error_context: ErrorContext<T, C>,
	pub data: D,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorData {
	pub msg: String,
	pub params: Option<HashMap<String, String>>,
}

#[allow(dead_code)]
pub type ActionRequestResult<T> = Result<T, Option<ErrorData>>;
