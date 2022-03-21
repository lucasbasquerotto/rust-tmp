use std::fmt::Debug;

use crate::business::action::data::user_action_data::UserRequestContext;

pub trait DescriptiveRequestContext: Debug + Clone {
	fn description(&self) -> String;
}

pub trait UserRequestContextLike {
	fn user_context(&self) -> UserRequestContext;
}