use std::fmt::Debug;

use crate::core::action::{
	action_type::{
		automatic_action_type::AutomaticActionType, moderator_action_type::ModeratorActionType,
		user_action_type::UserActionType,
	},
	data::{
		action_data::{DescriptiveError, ErrorData},
		automatic_action_data::{
			AutomaticActionInput, AutomaticErrorInfo, AutomaticOutputInfo, AutomaticRequestInput,
		},
		moderator_action_data::{
			ModeratorActionInput, ModeratorErrorInfo, ModeratorOutputInfo, ModeratorRequestInput,
		},
		user_action_data::{UserActionInput, UserErrorInfo, UserOutputInfo, UserRequestInput},
	},
};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

pub trait ActionInput: Debug {}

impl ActionInput for () {}

////////////////////////////////////////////////
//////////////////// OUTPUT ////////////////////
////////////////////////////////////////////////

pub trait ActionOutput: Debug {}

impl ActionOutput for () {}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

pub trait ActionError: Debug {
	fn private_error(&self) -> DescriptiveError;

	fn public_error(&self) -> Option<ErrorData>;

	fn error_msg(msg: String) -> Option<ErrorData> {
		Some(ErrorData { msg, params: None })
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

pub trait Action<I, O, E>: Debug
where
	Self: Sized,
{
	fn run(input: I) -> Result<O, E>;
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

pub trait UserAction<I, O, E>:
	Action<UserRequestInput<I>, UserOutputInfo<O>, UserErrorInfo<E>>
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError,
	Self: Sized,
{
	fn action_type() -> UserActionType;
	fn new(input: UserActionInput<I>) -> Result<Self, E>;
	fn run_inner(self) -> Result<O, E>;
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

pub trait ModeratorAction<I, O, E>:
	Action<ModeratorRequestInput<I>, ModeratorOutputInfo<O>, ModeratorErrorInfo<E>>
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError,
	Self: Sized,
{
	fn action_type() -> ModeratorActionType;
	fn new(input: ModeratorActionInput<I>) -> Result<Self, E>;
	fn run_inner(self) -> Result<O, E>;
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

pub trait AutomaticAction<I, O, E>:
	Action<AutomaticRequestInput<I>, AutomaticOutputInfo<O>, AutomaticErrorInfo<E>>
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError,
	Self: Sized,
{
	fn action_type() -> AutomaticActionType;
	fn new(input: AutomaticActionInput<I>) -> Result<Self, E>;
	fn run_inner(self) -> Result<O, E>;
}
