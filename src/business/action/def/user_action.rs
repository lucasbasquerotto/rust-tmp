use std::fmt::Debug;

use crate::{
	business::action::{
		business_action::{ActionResult, BusinessAction, BusinessException, ErrorData},
		r#type::user_action_type::{UserActionType, UserRequestInfo},
	},
	lib::{base::action::Action, core::action_core::RequestInput},
};

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
