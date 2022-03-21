use std::fmt::Debug;

use crate::business::action::data::{
	action_data::RequestContext, user_action_data::UserRequestContext,
};

pub trait DescriptiveRequestContext: Debug + Clone {
	fn description(&self) -> String;
}

impl<T: DescriptiveRequestContext> RequestContext for T {}

pub trait UserRequestContextLike {
	fn user_context(&self) -> UserRequestContext;
}
