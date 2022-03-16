use std::fmt::Debug;

use crate::{
	business::action::{
		action_data::{BusinessException, ErrorData},
		action_type::user_action_type::{UserActionType, UserRequestContext},
	},
	lib::{
		core::action::Action,
		core::action::{ActionType, RequestInput},
	},
};

pub type UserActionResult<T> = Result<T, BusinessException<UserRequestContext>>;

pub trait UserAction<I: Debug, O: Debug>: Debug {
	fn action_type() -> UserActionType;
	fn new(input: RequestInput<I, UserRequestContext>) -> Self;
	fn input(&self) -> &RequestInput<I, UserRequestContext>;
	fn run_inner(self) -> UserActionResult<O>;
}

// pub trait BusinessAction<I: Debug, O: Debug>: Debug
// where
// 	C: RequestContext,
// 	I: Debug,
// 	O: Debug,
// 	E: Debug,
// 	X: Exception<E>,
// 	D: Debug + Eq + PartialEq,
// 	T: ActionType<C, E, X, D>,
// {
// 	fn action_type() -> UserActionType;
// 	fn new(input: RequestInput<I, UserRequestContext>) -> Self;
// 	fn input(&self) -> &RequestInput<I, UserRequestContext>;
// 	fn run(self) -> UserActionResult<O>;
// }

impl<I, O, T>
	Action<
		UserRequestContext,
		I,
		O,
		Option<ErrorData>,
		BusinessException<UserRequestContext>,
		u32,
		UserActionType,
	> for T
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
		Self::action_type().validate(&self.input().context)?;
		self.run_inner()
	}
}
