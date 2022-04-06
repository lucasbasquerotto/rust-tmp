use std::fmt::Debug;

use crate::core::action::{
	action_type::{
		automatic_action_type::AutomaticActionType, moderator_action_type::ModeratorActionType,
		user_action_type::UserActionType,
	},
	data::{
		action_data::{ActionErrorInfo, DescriptiveError, ErrorData, RequestInput},
		automatic_action_data::{AutomaticActionError, AutomaticRequestContext},
		moderator_action_data::{ModeratorActionError, ModeratorRequestContext},
		user_action_data::{UserActionError, UserRequestContext},
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
	Action<
	RequestInput<I, UserRequestContext>,
	O,
	ActionErrorInfo<UserActionType, UserRequestContext, E>,
>
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError,
	Self: Sized,
{
	fn action_type() -> UserActionType;
	fn new(input: Result<RequestInput<I, UserRequestContext>, UserActionError>) -> Result<Self, E>;
	fn run_inner(self) -> Result<O, E>;
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

pub trait ModeratorAction<I, O, E>:
	Action<
	RequestInput<I, ModeratorRequestContext>,
	O,
	ActionErrorInfo<ModeratorActionType, ModeratorRequestContext, E>,
>
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError,
	Self: Sized,
{
	fn action_type() -> ModeratorActionType;
	fn new(
		input: Result<RequestInput<I, ModeratorRequestContext>, ModeratorActionError>,
	) -> Result<Self, E>;
	fn run_inner(self) -> Result<O, E>;
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

pub trait AutomaticAction<I, O, E>:
	Action<
	RequestInput<I, AutomaticRequestContext>,
	O,
	ActionErrorInfo<AutomaticActionType, AutomaticRequestContext, E>,
>
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError,
	Self: Sized,
{
	fn action_type() -> AutomaticActionType;
	fn new(
		input: Result<RequestInput<I, AutomaticRequestContext>, AutomaticActionError>,
	) -> Result<Self, E>;
	fn run_inner(self) -> Result<O, E>;
}
