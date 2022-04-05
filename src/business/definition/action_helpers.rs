use std::fmt::Debug;

use crate::data::{
	action::action_data::{ErrorData, RequestContext},
	action_type::general_action_type::ActionType,
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
