use std::{collections::HashMap, fmt::Debug};

use crate::lib::{
	base::action::Exception,
	core::action_core::{ActionContext, ActionType, RequestInfo, RequestInput},
};

pub trait ActionWithId<T> {
	fn id(&self) -> T;
}

#[derive(Debug)]
pub struct BusinessException {
	pub private: Option<ErrorData>,
	pub public: Option<ErrorData>,
}

#[derive(Debug)]
pub struct ErrorData {
	pub key: &'static str,
	pub msg: &'static str,
	pub params: Option<HashMap<String, String>>,
	pub meta: Option<HashMap<String, String>>,
}

impl Exception<Option<ErrorData>> for BusinessException {
	fn handle(self) -> Option<ErrorData> {
		//TODO log
		println!(
			"error: {private:?} / {public:?}",
			private = &self.private,
			public = &self.public
		);
		self.public
	}
}

pub type ActionResult<T> = Result<T, BusinessException>;

pub type ActionRequestResult<T> = Result<T, Option<ErrorData>>;

pub trait BusinessActionType<T: RequestInfo>: PartialEq + Eq + Debug {
	fn context() -> ActionContext;
	fn validate(input: T) -> Result<(), BusinessException>;
}

impl<I, T> ActionType<I, BusinessException> for T
where
	I: RequestInfo,
	T: BusinessActionType<I>,
{
	fn context() -> ActionContext {
		Self::context()
	}

	fn validate(input: I) -> Result<(), BusinessException> {
		Self::validate(input)
	}
}

pub trait BusinessAction<I: RequestInfo, D: Debug, O: Debug, T: BusinessActionType<I>> {
	fn action_type() -> T;
	fn new(input: RequestInput<D, I>) -> Self;
	fn run(self) -> ActionResult<O>;
}

#[derive(Debug)]
pub struct Request {
	pub ip: String,
}

#[derive(Debug)]
pub struct Session {
	pub user_id: u64,
}

#[derive(Debug)]
pub struct ModeratorSession {
	pub user_id: u64,
	pub allowed_actions: u32,
}

#[derive(Debug)]
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
