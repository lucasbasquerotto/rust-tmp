use std::fmt::Debug;

use crate::business::{
	action_type::action_type::ActionType,
	data::action_data::{ErrorData, RequestContext},
};

pub trait DescriptiveRequestContext: Debug + Eq + PartialEq + Clone {
	fn description(&self) -> String;
}

pub trait ActionErrorHelper<T: ActionType, C: RequestContext>: Debug
where
	Self: Sized,
{
	fn default_description(&self) -> String;

	fn error_msg(&self, msg: String) -> Option<ErrorData>;

	fn type_of<K>(_: &K) -> String;
}
