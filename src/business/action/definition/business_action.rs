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
	lib::core::action::{ActionScope, RequestContext, RequestInput},
};

pub type UserActionResult<T> = Result<T, BusinessException<UserRequestContext>>;

pub trait ActionInput: Debug {}

pub trait ActionOutput: Debug {}

impl ActionInput for () {}

impl ActionOutput for () {}

pub trait BusinessActionType<C: RequestContext>: Debug + Eq + PartialEq {
	fn scope() -> ActionScope;
}

// pub trait BusinessAction<I: ActionInput, O: ActionOutput, C: ActionType<C, E, X, D>>: Debug {}

pub trait UserAction<I: ActionInput, O: ActionOutput>
where
	Self: Sized,
{
	fn action_type() -> UserActionType;
	fn new(input: RequestInput<I, UserRequestContext>) -> UserActionResult<Self>;
	fn run_inner(self) -> UserActionResult<O>;
}

// impl<I: ActionInput, O: ActionOutput, T: UserAction<I, O>> BusinessAction<I, O> for T {}

pub type ModeratorActionResult<T> = Result<T, BusinessException<ModeratorRequestContext>>;

pub trait ModeratorAction<I: ActionInput, O: ActionOutput>
where
	Self: Sized,
{
	fn action_type() -> ModeratorActionType;
	fn new(input: RequestInput<I, ModeratorRequestContext>) -> ModeratorActionResult<Self>;
	fn run_inner(self) -> ModeratorActionResult<O>;
}

// impl<I: ActionInput, O: ActionOutput, T: ModeratorAction<I, O>> BusinessAction<I, O> for T {}
