use std::fmt::Debug;

use crate::{
	core::action::{
		action_type::general_action_type::ActionType,
		data::action_data::{ErrorData, RequestContext},
	},
	lib::data::str::Str,
};

use super::action::ActionError;

pub trait DescriptiveInfo {
	fn description(&self) -> Str;
}

pub trait DescriptiveRequestContext:
	Debug + Eq + PartialEq + Clone + DescriptiveInfo + RequestContext
{
}

impl<T: Debug + Eq + PartialEq + Clone + DescriptiveInfo + RequestContext> DescriptiveRequestContext
	for T
{
}

pub trait ActionErrorHelper<T: ActionType, C: RequestContext, E: ActionError>: Debug
where
	Self: Sized,
{
	fn description(&self) -> Str;

	fn handle(self) -> Option<ErrorData>;
}
