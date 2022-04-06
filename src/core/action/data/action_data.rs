use std::{collections::HashMap, fmt::Debug};

use crate::core::action::action_type::general_action_type::ActionType;

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

#[cfg(not(test))]
#[derive(Debug)]
pub struct ErrorInfo<D: Debug + Eq + PartialEq, E: Debug = Option<()>> {
	pub data: D,
	pub source: E,
}

#[cfg(test)]
#[derive(Debug)]
pub struct ErrorInfo<D: Debug + Eq + PartialEq, E: Debug = ()> {
	pub data: D,
	pub source: Option<E>,
}

#[cfg(test)]
impl<D: Debug + Eq + PartialEq, E: Debug> ErrorInfo<D, E> {
	pub fn mock(data: D) -> Self {
		Self { data, source: None }
	}
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
pub struct DescriptiveError {
	pub msg: Option<String>,
	pub data: Option<String>,
	pub source: Option<String>,
}

impl DescriptiveError {
	pub fn empty() -> Self {
		Self {
			msg: None,
			data: None,
			source: None,
		}
	}

	pub fn data<T: Debug>(data: T) -> Self {
		Self {
			msg: None,
			data: Some(format!("{data:?}")),
			source: None,
		}
	}

	pub fn source<T: Debug>(source: T) -> Self {
		Self {
			msg: None,
			data: None,
			source: Some(format!("{source:?}")),
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorData {
	pub msg: String,
	pub params: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct ActionErrorInfo<T: ActionType, C: RequestContext, E> {
	pub error_context: ErrorContext<T, C>,
	pub error: E,
}
