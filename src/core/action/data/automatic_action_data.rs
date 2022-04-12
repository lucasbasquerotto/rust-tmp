use crate::core::action::{
	action_type::automatic_action_type::AutomaticActionType,
	data::action_data::{
		ActionErrorInfo, ActionResultInfo, Application, Request, RequestContext, RequestInput,
	},
	definition::action::ActionResult,
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

impl RequestContext for AutomaticRequestContext {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InternalRequestContext {
	pub application: Application,
}

impl RequestContext for InternalRequestContext {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HookRequestContext {
	pub application: Application,
	pub request: Request,
}

impl RequestContext for HookRequestContext {}

pub type AutomaticRequestInput<I> = RequestInput<I, AutomaticRequestContext>;

pub type InternalRequestInput<I> = RequestInput<I, InternalRequestContext>;

pub type HookRequestInput<I> = RequestInput<I, HookRequestContext>;

pub type AutomaticActionInput<I> = ActionResult<AutomaticRequestInput<I>, AutomaticActionError>;

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
	use crate::core::action::data::action_data::{
		tests::{ApplicationBuilder, RequestBuilder},
		Application,
	};

	use super::{AutomaticRequest, AutomaticRequestContext};

	#[allow(dead_code)]
	pub struct AutomaticRequestContextBuilder(AutomaticRequestContext);

	#[allow(dead_code)]
	impl AutomaticRequestContextBuilder {
		pub fn new() -> Self {
			Self(AutomaticRequestContext {
				application: ApplicationBuilder::new().build(),
				request: AutomaticRequest::Internal,
			})
		}

		pub fn application(mut self, application: Application) -> Self {
			self.0.application = application;
			self
		}

		pub fn request(mut self, request: AutomaticRequest) -> Self {
			self.0.request = request;
			self
		}

		pub fn build(self) -> AutomaticRequestContext {
			self.0
		}

		pub fn build_internal() -> AutomaticRequestContext {
			Self::new().build()
		}

		pub fn build_hook() -> AutomaticRequestContext {
			Self::new()
				.request(AutomaticRequest::Hook(RequestBuilder::new().build()))
				.build()
		}
	}
}
