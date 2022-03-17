use std::fmt::Debug;

use crate::{
	business::action::{
		action_data::general_action_data::{BusinessException, ErrorData},
		action_data::moderator_action_data::{ModeratorActionType, ModeratorRequestContext},
	},
	lib::{
		core::action::Action,
		core::action::{ActionType, RequestInput},
	},
};

pub type ModeratorActionResult<T> = Result<T, BusinessException<ModeratorRequestContext>>;

pub trait ModeratorAction<I: Debug, O: Debug>: Debug {
	fn action_type() -> ModeratorActionType;
	fn new(input: RequestInput<I, ModeratorRequestContext>) -> Self;
	fn input(&self) -> &RequestInput<I, ModeratorRequestContext>;
	fn run_inner(self) -> ModeratorActionResult<O>;
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
		ModeratorRequestContext,
		I,
		O,
		Option<ErrorData>,
		BusinessException<ModeratorRequestContext>,
		u32,
		ModeratorActionType,
	> for T
where
	I: Debug,
	O: Debug,
	T: ModeratorAction<I, O>,
{
	fn action_type() -> ModeratorActionType {
		Self::action_type()
	}

	fn new(input: RequestInput<I, ModeratorRequestContext>) -> Self {
		Self::new(input)
	}

	fn input(&self) -> &RequestInput<I, ModeratorRequestContext> {
		self.input()
	}

	fn run(self) -> ModeratorActionResult<O> {
		Self::action_type().validate(&self.input().context)?;
		self.run_inner()
	}
}
