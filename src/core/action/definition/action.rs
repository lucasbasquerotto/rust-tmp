use std::fmt::Debug;

use crate::{
	core::action::{
		action_type::{
			automatic_action_type::AutomaticActionType, moderator_action_type::ModeratorActionType,
			user_action_type::UserActionType,
		},
		data::{
			action_data::{DescriptiveError, ErrorData},
			automatic_action_data::{
				AutomaticActionError, AutomaticActionInput, AutomaticErrorInfo,
				AutomaticOutputInfo, AutomaticRequestInput,
			},
			moderator_action_data::{
				ModeratorActionError, ModeratorActionInput, ModeratorErrorInfo,
				ModeratorOutputInfo, ModeratorRequestInput,
			},
			user_action_data::{
				UserActionError, UserActionInput, UserErrorInfo, UserOutputInfo, UserRequestInput,
			},
		},
	},
	lib::data::str::Str,
};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

pub trait ActionInput: Debug + Send {}

impl ActionInput for () {}

////////////////////////////////////////////////
//////////////////// OUTPUT ////////////////////
////////////////////////////////////////////////

pub trait ActionOutput: Debug {}

impl ActionOutput for () {}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

pub trait ActionError: Debug + Send {
	fn private_error(&self) -> Option<DescriptiveError>;

	fn public_error(&self) -> Option<ErrorData>;

	fn error_msg(msg: Str) -> Option<ErrorData> {
		Some(ErrorData { msg, params: None })
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[rocket::async_trait]
pub trait Action<I, O, E>: Debug
where
	Self: Sized,
{
	async fn run(input: I) -> Result<O, E>;
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[rocket::async_trait]
pub trait UserAction<I, O, E>:
	Action<UserActionInput<I>, UserOutputInfo<O>, UserErrorInfo<E>>
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError + From<UserActionError>,
	Self: Sized,
{
	fn action_type() -> UserActionType;
	async fn new(input: UserRequestInput<I>) -> Result<Self, E>;
	async fn run_inner(self) -> Result<O, E>;
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[rocket::async_trait]
pub trait ModeratorAction<I, O, E>:
	Action<ModeratorActionInput<I>, ModeratorOutputInfo<O>, ModeratorErrorInfo<E>>
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError + From<ModeratorActionError>,
	Self: Sized,
{
	fn action_type() -> ModeratorActionType;
	async fn new(input: ModeratorRequestInput<I>) -> Result<Self, E>;
	async fn run_inner(self) -> Result<O, E>;
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[rocket::async_trait]
pub trait AutomaticAction<I, O, E>:
	Action<AutomaticActionInput<I>, AutomaticOutputInfo<O>, AutomaticErrorInfo<E>>
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError + From<AutomaticActionError>,
	Self: Sized,
{
	fn action_type() -> AutomaticActionType;
	async fn new(input: AutomaticRequestInput<I>) -> Result<Self, E>;
	async fn run_inner(self) -> Result<O, E>;
}
