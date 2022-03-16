use std::fmt::Debug;

use crate::{
	business::action::{
		action_data::{BusinessException, ErrorData},
		action_type::user_action_type::{UserActionType, UserRequestContext},
		business_action::{BusinessAction, BusinessActionType},
	},
	lib::{core::action::Action, core::action::RequestInput},
};

pub type UserActionResult<T> = Result<T, BusinessException<UserRequestContext>>;

pub trait UserAction<I: Debug, O: Debug>: Debug {
	fn action_type() -> UserActionType;
	fn new(input: RequestInput<I, UserRequestContext>) -> Self;
	fn input(&self) -> &RequestInput<I, UserRequestContext>;
	fn run(self) -> UserActionResult<O>;
}

impl<I, O, T>
	BusinessAction<UserRequestContext, I, O, BusinessException<UserRequestContext>, UserActionType>
	for T
where
	I: Debug,
	O: Debug,
	T: UserAction<I, O>,
{
	fn action_type() -> UserActionType {
		Self::action_type()
	}

	fn new(input: RequestInput<I, UserRequestContext>) -> Self {
		Self::new(input)
	}

	fn input(&self) -> &RequestInput<I, UserRequestContext> {
		self.input()
	}

	fn run(self) -> UserActionResult<O> {
		self.run()
	}
}

impl<I, O, T>
	Action<
		RequestInput<I, UserRequestContext>,
		O,
		Option<ErrorData>,
		BusinessException<UserRequestContext>,
	> for T
where
	I: Debug,
	O: Debug,
	T: BusinessAction<
		UserRequestContext,
		I,
		O,
		BusinessException<UserRequestContext>,
		UserActionType,
	>,
{
	fn new(input: RequestInput<I, UserRequestContext>) -> Self {
		Self::new(input)
	}

	fn run(self) -> UserActionResult<O> {
		Self::action_type().validate(&self.input().context)?;
		self.run()
	}
}
