use crate::core::action::data::action_data::{Application, Request};
use std::fmt::Debug;

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AutomaticRequest {
	Internal,
	Hook(Request),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutomaticRequestContext {
	pub application: Application,
	pub request: AutomaticRequest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InternalRequestContext {
	pub application: Application,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HookRequestContext {
	pub application: Application,
	pub request: Request,
}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq)]
pub enum AutomaticActionError {
	NotInternal,
	NotHook,
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use crate::core::action::data::action_data::{Application, Request};

	use super::{AutomaticRequest, AutomaticRequestContext};

	#[derive(Debug, Clone)]
	pub struct AutomaticTestOptions {
		pub internal: bool,
	}

	pub fn automatic_context(options: AutomaticTestOptions) -> AutomaticRequestContext {
		AutomaticRequestContext {
			application: Application {
				request_timeout: 1000,
			},
			request: if options.internal {
				AutomaticRequest::Internal
			} else {
				AutomaticRequest::Hook(Request {
					ip: "0.1.2.3".to_string(),
				})
			},
		}
	}
}
