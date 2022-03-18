use std::fmt::Debug;

use crate::{
	business::action::{
		action_type::{
			moderator_action_type::ModeratorActionType, user_action_type::UserActionType,
		},
		data::{
			action_data::BusinessException, moderator_action_data::ModeratorRequestContext,
			user_action_data::UserRequestContext,
		},
	},
	lib::core::action::RequestInput,
};

pub type UserActionResult<T> = Result<T, BusinessException<UserRequestContext>>;

pub trait UserAction<I: Debug, O: Debug>: Debug {
	fn action_type() -> UserActionType;
	fn new(input: RequestInput<I, UserRequestContext>) -> Self;
	fn input(&self) -> &RequestInput<I, UserRequestContext>;
	fn run_inner(self) -> UserActionResult<O>;
}

pub type ModeratorActionResult<T> = Result<T, BusinessException<ModeratorRequestContext>>;

pub trait ModeratorAction<I: Debug, O: Debug>: Debug {
	fn action_type() -> ModeratorActionType;
	fn new(input: RequestInput<I, ModeratorRequestContext>) -> Self;
	fn input(&self) -> &RequestInput<I, ModeratorRequestContext>;
	fn run_inner(self) -> ModeratorActionResult<O>;
}
