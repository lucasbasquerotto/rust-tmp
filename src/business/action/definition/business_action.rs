use std::fmt::Debug;

use crate::business::action::{
	action_type::{moderator_action_type::ModeratorActionType, user_action_type::UserActionType},
	data::{
		action_data::RequestInput,
		moderator_action_data::{ModeratorActionError, ModeratorRequestContext},
		user_action_data::{UserActionError, UserRequestContext},
	},
};

/////////////////////////////////////////////////////////
// Input + Output
/////////////////////////////////////////////////////////

pub trait ActionInput: Debug {}

pub trait ActionOutput: Debug {}

pub trait ActionError: Debug {}

impl ActionInput for () {}

impl ActionOutput for () {}

impl ActionError for () {}

pub trait Action<I, O, E>: Debug
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError,
	Self: Sized,
{
	fn new(input: I) -> Result<Self, E>;
	fn run(self) -> Result<O, E>;
}

/////////////////////////////////////////////////////////
// User Action
/////////////////////////////////////////////////////////

pub trait UserAction<I, O, E>: Action<RequestInput<I, UserRequestContext>, O, E>
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError,
	Self: Sized,
{
	fn action_type() -> UserActionType;
	fn new_inner(
		input: Result<RequestInput<I, UserRequestContext>, UserActionError>,
	) -> Result<Self, E>;
	fn run_inner(self) -> Result<O, E>;
}

/////////////////////////////////////////////////////////
// Moderator Action
/////////////////////////////////////////////////////////

pub trait ModeratorAction<I, O, E>: Action<RequestInput<I, ModeratorRequestContext>, O, E>
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError,
	Self: Sized,
{
	fn action_type() -> ModeratorActionType;
	fn new_inner(
		input: Result<RequestInput<I, ModeratorRequestContext>, ModeratorActionError>,
	) -> Result<Self, E>;
	fn run_inner(self) -> Result<O, E>;
}
