use crate::core::action::{
	action_type::automatic_action_type::AutomaticActionType,
	data::action_data::{ActionErrorInfo, ActionResultInfo, Application, Request, RequestInput},
};
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

pub type AutomaticRequestInput<I> = RequestInput<I, AutomaticRequestContext>;

pub type InternalRequestInput<I> = RequestInput<I, InternalRequestContext>;

pub type HookRequestInput<I> = RequestInput<I, HookRequestContext>;

pub type AutomaticActionInput<I> = Result<AutomaticRequestInput<I>, AutomaticActionError>;

pub type AutomaticOutputInfo<D> = ActionResultInfo<AutomaticActionType, AutomaticRequestContext, D>;

pub type AutomaticErrorInfo<E> = ActionErrorInfo<AutomaticActionType, AutomaticRequestContext, E>;

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
