use std::fmt::Debug;

use crate::{
	business::action::{
		action_type::{
			moderator_action_type::ModeratorActionType, user_action_type::UserActionType,
		},
		data::{
			moderator_action_data::{ModeratorActionError, ModeratorRequestContext},
			user_action_data::{UserActionError, UserRequestContext},
		},
	},
	lib::core::action::RequestInput,
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

/////////////////////////////////////////////////////////
// User Action
/////////////////////////////////////////////////////////

pub trait UserAction<I: ActionInput, O: ActionOutput, E: ActionError>: Debug
where
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

pub trait ModeratorAction<I: ActionInput, O: ActionOutput, E: ActionError>: Debug
where
	Self: Sized,
{
	fn action_type() -> ModeratorActionType;
	fn new_inner(
		input: Result<RequestInput<I, ModeratorRequestContext>, ModeratorActionError>,
	) -> Result<Self, E>;
	fn run_inner(self) -> Result<O, E>;
}
