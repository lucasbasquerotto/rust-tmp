use crate::business::action_type::automatic_action_type::AutomaticActionType;

use super::action_data::{Application, ErrorInput, Request};

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

pub type AutomaticErrorInput<T> = ErrorInput<AutomaticActionType, AutomaticRequestContext, T>;

#[derive(Debug, Eq, PartialEq)]
pub enum AutomaticActionError {
	NotInternal(AutomaticErrorInput<()>),
	NotHook(AutomaticErrorInput<()>),
}

#[cfg(test)]
pub mod tests {
	use business::data::action_data::{Application, Request};

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
