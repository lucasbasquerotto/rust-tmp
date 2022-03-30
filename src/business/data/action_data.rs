use std::{collections::HashMap, fmt::Debug};

use crate::business::action_type::action_type::ActionType;

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

pub trait RequestContext: Clone + Debug + Eq + PartialEq {}

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

////////////////////////////////////////////////
//////////////////// OUTPUT ////////////////////
////////////////////////////////////////////////

#[allow(dead_code)]
pub type ActionRequestResult<T> = Result<T, Option<ErrorData>>;

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorContext<T: ActionType, C: RequestContext> {
	pub action_type: T,
	pub context: C,
}

#[derive(Debug)]
pub struct ErrorInfo<D: Debug + Eq + PartialEq, E: Debug = Option<()>> {
	pub data: D,
	pub source: E,
}

#[derive(Debug)]
pub struct ErrorContextInfo<
	D: Debug + Eq + PartialEq,
	T: ActionType,
	C: RequestContext,
	E: Debug = Option<()>,
> {
	pub error_context: ErrorContext<T, C>,
	pub error_info: ErrorInfo<D, E>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DescriptiveError<T: ActionType, C: RequestContext> {
	pub error_context: ErrorContext<T, C>,
	pub data: String,
	pub source: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorData {
	pub msg: String,
	pub params: Option<HashMap<String, String>>,
}

#[cfg(not(test))]
#[derive(Debug)]
pub struct ErrorBox<T, E> {
	pub data: T,
	pub error: E,
}

#[cfg(test)]
#[derive(Debug)]
pub struct ErrorBox<T, E> {
	pub data: Option<T>,
	pub error: Option<E>,
}

#[cfg(test)]
impl<T, E> ErrorBox<T, E> {
	pub fn mock() -> Self {
		Self {
			data: None,
			error: None,
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub enum ActionScope {
	User,
	Moderator,
	Automatic,
}
