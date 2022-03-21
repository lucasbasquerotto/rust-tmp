use std::fmt::Debug;

use crate::business::action::{action_type::action_type::ActionType, data::action_data::ErrorData};

use super::action_helpers::DescriptiveRequestContext;

pub trait ActionErrorHelper<T: ActionType, C: DescriptiveRequestContext>: Debug
where
	Self: Sized,
{
	fn default_description(&self) -> String;

	fn handle(self) -> Option<ErrorData>;

	fn error_msg(&self, msg: String) -> Option<ErrorData>;

	fn type_of<K>(_: &K) -> String;

	fn info(&self);

	fn warn(&self);

	fn error(&self);

	fn debug(&self);
}
