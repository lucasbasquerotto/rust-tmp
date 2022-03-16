use std::fmt::Debug;

use crate::lib::{
	base::action::Exception,
	core::action_core::{ActionContext, ActionType, RequestInfo, RequestInput},
};

use super::action_data::{BusinessException, ErrorData};

pub trait BusinessActionType<T, I>: PartialEq + Eq + Debug
where
	T: RequestInfo,
	I: PartialEq + Eq + Debug,
{
	fn context() -> ActionContext;
	fn id(&self) -> I;
	fn validate(&self, info: &T) -> Result<(), BusinessException<T>>;
}

impl<I, T> ActionType<I, BusinessException<I>> for T
where
	I: RequestInfo,
	T: BusinessActionType<I, u32>,
{
	fn context() -> ActionContext {
		Self::context()
	}

	fn validate(&self, info: &I) -> Result<(), BusinessException<I>> {
		self.validate(info)
	}
}

pub trait BusinessAction<
	I: RequestInfo,
	D: Debug,
	O: Debug,
	E: Exception<Option<ErrorData>>,
	T: BusinessActionType<I, u32>,
>
{
	fn action_type() -> T;
	fn new(input: RequestInput<D, I>) -> Self;
	fn run(self) -> Result<O, E>;
}

// #[derive(Debug)]
// pub struct ModeratorRequestInfo {
// 	pub application: Application,
// 	pub session: ModeratorSession,
// 	pub request: Request,
// }

// impl RequestInfo for ModeratorRequestInfo {}

// #[derive(Debug)]
// pub struct AutomaticRequestInfo {
// 	pub application: Application,
// 	pub request: Request,
// }

// impl RequestInfo for AutomaticRequestInfo {}
