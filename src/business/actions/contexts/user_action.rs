use std::fmt::Debug;

use crate::{
	business::actions::business_action::{
		ActionResult, ActionWithId, Application, BusinessAction, BusinessActionType,
		BusinessException, ErrorData, Request, Session,
	},
	lib::{
		base::action::Action,
		core::action_core::{ActionContext, RequestInfo, RequestInput},
	},
};

#[derive(Debug, PartialEq, Eq)]
pub enum UserActionType {
	LOGIN,
	LOGOUT,
}

impl ActionWithId<u32> for UserActionType {
	fn id(&self) -> u32 {
		match self {
			UserActionType::LOGIN => 1,
			UserActionType::LOGOUT => 2,
		}
	}
}

#[derive(Debug)]
pub struct UserRequestInfo {
	pub application: Application,
	pub session: Session,
	pub request: Request,
}

impl RequestInfo for UserRequestInfo {}

impl BusinessActionType<UserRequestInfo> for UserActionType {
	fn context() -> ActionContext {
		ActionContext::USER
	}

	fn validate(_: UserRequestInfo) -> Result<(), BusinessException> {
		//TODO
		Ok(())
	}
}

pub trait UserAction<I: Debug, O: Debug>: Debug {
	fn action_type() -> UserActionType;
	fn new(input: RequestInput<I, UserRequestInfo>) -> Self;
	fn run(self) -> ActionResult<O>;
}

impl<I, O, T> BusinessAction<UserRequestInfo, I, O, UserActionType> for T
where
	I: Debug,
	O: Debug,
	T: UserAction<I, O>,
{
	fn action_type() -> UserActionType {
		Self::action_type()
	}

	fn new(input: RequestInput<I, UserRequestInfo>) -> Self {
		Self::new(input)
	}

	fn run(self) -> ActionResult<O> {
		self.run()
	}
}

impl<I, O, T> Action<RequestInput<I, UserRequestInfo>, O, Option<ErrorData>, BusinessException>
	for T
where
	I: Debug,
	O: Debug,
	T: BusinessAction<UserRequestInfo, I, O, UserActionType>,
{
	fn new(input: RequestInput<I, UserRequestInfo>) -> Self {
		Self::new(input)
	}

	fn run(self) -> ActionResult<O> {
		self.run()
	}
}
