use std::fmt::Debug;

use crate::{
	business::action::{
		action_type::user_action_type::{UserActionType, UserRequestInfo},
		business_action::{BusinessAction, BusinessActionType, BusinessException, ErrorData},
	},
	lib::{base::action::Action, core::action_core::RequestInput},
};

pub type UserActionResult<T> = Result<T, BusinessException<UserRequestInfo>>;

pub trait UserAction<I: Debug, O: Debug>: Debug {
	fn action_type() -> UserActionType;
	fn new(input: RequestInput<I, UserRequestInfo>) -> Self;
	fn input(&self) -> &RequestInput<I, UserRequestInfo>;
	fn run(self) -> UserActionResult<O>;
}

impl<I, O, T>
	BusinessAction<UserRequestInfo, I, O, BusinessException<UserRequestInfo>, UserActionType> for T
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

	fn run(self) -> UserActionResult<O> {
		Self::action_type().validate(&self.input().info)?;
		self.run()
	}
}

impl<I, O, T>
	Action<
		RequestInput<I, UserRequestInfo>,
		O,
		Option<ErrorData>,
		BusinessException<UserRequestInfo>,
	> for T
where
	I: Debug,
	O: Debug,
	T: BusinessAction<UserRequestInfo, I, O, BusinessException<UserRequestInfo>, UserActionType>,
{
	fn new(input: RequestInput<I, UserRequestInfo>) -> Self {
		Self::new(input)
	}

	fn run(self) -> UserActionResult<O> {
		self.run()
	}
}
