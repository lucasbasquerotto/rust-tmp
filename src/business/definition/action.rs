use std::fmt::Debug;

use crate::business::{
	action_type::{
		action_type::ActionType, automatic_action_type::AutomaticActionType,
		moderator_action_type::ModeratorActionType, user_action_type::UserActionType,
	},
	data::{
		action_data::{DescriptiveError, ErrorData, RequestContext, RequestInput},
		automatic_action_data::{AutomaticActionError, AutomaticRequestContext},
		moderator_action_data::{ModeratorActionError, ModeratorRequestContext},
		user_action_data::{UserActionError, UserRequestContext},
	},
};

use super::action_helpers::ActionErrorHelper;

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

pub trait ActionError<T: ActionType, C: RequestContext>: ActionErrorHelper<T, C>
where
	Self: Sized,
{
	fn error_input(&self) -> DescriptiveError<T, C>;

	fn public_error(&self) -> Option<ErrorData>;

	fn description(&self) -> String {
		self.default_description()
	}

	fn handle(self) -> Option<ErrorData> {
		error!("{}", self.description());
		self.public_error()
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

pub trait UserAction<I, O, E>: Action<RequestInput<I, UserRequestContext>, O, E>
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError<UserActionType, UserRequestContext>,
	Self: Sized,
{
	fn action_type() -> UserActionType;
	fn new(input: Result<RequestInput<I, UserRequestContext>, UserActionError>) -> Result<Self, E>;
	fn run_inner(self) -> Result<O, E>;
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

pub trait ModeratorAction<I, O, E>: Action<RequestInput<I, ModeratorRequestContext>, O, E>
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError<ModeratorActionType, ModeratorRequestContext>,
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

pub trait AutomaticAction<I, O, E>: Action<RequestInput<I, AutomaticRequestContext>, O, E>
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError<AutomaticActionType, AutomaticRequestContext>,
	Self: Sized,
{
	fn action_type() -> AutomaticActionType;
	fn new(
		input: Result<RequestInput<I, AutomaticRequestContext>, AutomaticActionError>,
	) -> Result<Self, E>;
	fn run_inner(self) -> Result<O, E>;
}
