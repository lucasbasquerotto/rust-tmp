use std::fmt::Debug;

use crate::business::{
	action_type::action_type::ActionType,
	data::action_data::{ErrorData, RequestContext},
};

use super::action::ActionError;

pub trait DescriptiveRequestContext: Debug + Eq + PartialEq + Clone {
	fn description(&self) -> String;
}

pub trait ActionErrorHelper<T: ActionType, C: RequestContext, E: ActionError>: Debug
where
	Self: Sized,
{
	fn description(&self) -> String;

	fn handle(self) -> Option<ErrorData>;
}
