use std::fmt::Debug;

use crate::business::action::{
	action_type::{
		action_type::ActionType, moderator_action_type::ModeratorActionType,
		user_action_type::UserActionType,
	},
	data::{
		action_data::{ErrorContext, ErrorData, RequestInput},
		moderator_action_data::{ModeratorActionError, ModeratorRequestContext},
		user_action_data::{UserActionError, UserRequestContext},
	},
};

use super::action_helpers::DescriptiveRequestContext;

/////////////////////////////////////////////////////////
// Input + Output + Error
/////////////////////////////////////////////////////////

pub trait ActionInput: Debug {}

pub trait ActionOutput: Debug {}

pub trait ActionError<T: ActionType, C: DescriptiveRequestContext>: Debug {
	fn error_context(&self) -> &ErrorContext<T, C>;

	fn public_error(&self) -> Option<ErrorData>;

	fn description(&self) -> String;
}

impl ActionInput for () {}

impl ActionOutput for () {}

pub trait Action<I, O, E>: Debug
where
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
	E: ActionError<UserActionType, UserRequestContext>,
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
	E: ActionError<ModeratorActionType, ModeratorRequestContext>,
	Self: Sized,
{
	fn action_type() -> ModeratorActionType;
	fn new_inner(
		input: Result<RequestInput<I, ModeratorRequestContext>, ModeratorActionError>,
	) -> Result<Self, E>;
	fn run_inner(self) -> Result<O, E>;
}