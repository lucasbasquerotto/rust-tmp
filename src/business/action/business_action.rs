use std::{collections::HashMap, fmt::Debug};

use crate::lib::{
	base::action::Exception,
	core::action_core::{ActionContext, ActionType, RequestInfo, RequestInput},
};

#[derive(Debug)]
pub struct BusinessException<T: RequestInfo> {
	pub info: Option<T>,
	pub private: Option<ErrorData>,
	pub public: Option<ErrorData>,
}

#[derive(Debug)]
pub struct ErrorData {
	pub key: String,
	pub msg: &'static str,
	pub params: Option<HashMap<String, String>>,
	pub meta: Option<HashMap<String, String>>,
}

pub type ActionRequestResult<T> = Result<T, Option<ErrorData>>;

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

#[derive(Clone, Debug)]
pub struct Request {
	pub ip: String,
}

#[derive(Clone, Debug)]
pub struct Session {
	pub user_id: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct ModeratorSession {
	pub user_id: u64,
	pub allowed_actions: u32,
}

#[derive(Clone, Debug)]
pub struct Application {
	pub request_timeout: u32,
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
