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
pub struct ErrorInput<D: Debug + Eq + PartialEq, T: ActionType, C: RequestContext, E: Debug = ()> {
	pub error_context: ErrorContext<T, C>,
	pub data: D,
	pub source: Option<E>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DescriptiveErrorInput<T: ActionType, C: RequestContext> {
	pub error_context: ErrorContext<T, C>,
	pub data: String,
	pub source: String,
}

impl<D: Debug + Eq + PartialEq, T: ActionType, C: RequestContext, E: Debug> ErrorInput<D, T, C, E> {
	pub fn to_descriptive(&self) -> DescriptiveErrorInput<T, C> {
		let Self {
			error_context,
			data,
			source,
		} = self;

		DescriptiveErrorInput {
			error_context: error_context.clone(),
			data: format!("{data:?}"),
			source: format!("{source:?}"),
		}
	}
}

impl<T: ActionType, C: RequestContext> ErrorContext<T, C> {
	#[allow(dead_code)]
	pub fn to_descriptive(&self) -> DescriptiveErrorInput<T, C> {
		DescriptiveErrorInput {
			error_context: self.clone(),
			data: "".to_string(),
			source: "".to_string(),
		}
	}
}

impl<D: Debug + Eq + PartialEq, T: ActionType, C: RequestContext, E: Debug> PartialEq
	for ErrorInput<D, T, C, E>
{
	fn eq(&self, other: &Self) -> bool {
		self.error_context == other.error_context && self.data == other.data
	}
}

impl<D: Debug + Eq + PartialEq, T: ActionType, C: RequestContext> Eq for ErrorInput<D, T, C> {}

#[derive(Debug, Eq, PartialEq)]
pub struct ErrorData {
	pub msg: String,
	pub params: Option<HashMap<String, String>>,
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
