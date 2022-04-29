use std::{collections::HashMap, fmt::Debug};

use crate::{core::action::action_type::general_action_type::ActionType, lib::data::str::Str};

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
	pub ip: Str,
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
pub struct ActionContext<T: ActionType, C: RequestContext> {
	pub action_type: T,
	pub context: Option<C>,
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
	pub error_context: ActionContext<T, C>,
	pub error_info: ErrorInfo<D, E>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DescriptiveError {
	pub msg: Option<Str>,
	pub data: Option<Str>,
	pub source: Option<Str>,
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
			data: Some(format!("{data:?}").into()),
			source: None,
		}
	}

	pub fn source<T: Debug>(source: T) -> Self {
		Self {
			msg: None,
			data: None,
			source: Some(format!("{source:?}").into()),
		}
	}
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ErrorData {
	pub msg: Str,
	pub params: Option<HashMap<Str, Str>>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ActionResultInfo<T: ActionType, C: RequestContext, D> {
	pub action_context: ActionContext<T, C>,
	pub data: D,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ActionErrorInfo<T: ActionType, C: RequestContext, E> {
	pub action_context: ActionContext<T, C>,
	pub error: E,
}

#[cfg(test)]
pub mod tests {
	use crate::lib::data::str::Str;

	use super::{Application, Request};

	#[allow(dead_code)]
	pub struct ApplicationBuilder(Application);

	#[allow(dead_code)]
	impl ApplicationBuilder {
		pub fn new() -> Self {
			Self(Application {
				request_timeout: 1000,
			})
		}

		pub fn request_timeout(mut self, request_timeout: u32) -> Self {
			self.0.request_timeout = request_timeout;
			self
		}

		pub fn build(self) -> Application {
			self.0
		}
	}

	#[allow(dead_code)]
	pub struct RequestBuilder(Request);

	#[allow(dead_code)]
	impl RequestBuilder {
		pub fn new() -> Self {
			Self(Request { ip: "".into() })
		}

		pub fn ip(mut self, ip: Str) -> Self {
			self.0.ip = ip;
			self
		}

		pub fn build(self) -> Request {
			self.0
		}
	}
}
