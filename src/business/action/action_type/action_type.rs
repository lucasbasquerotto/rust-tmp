use std::fmt::Debug;

use crate::{
	business::action::definition::action_helpers::DescriptiveRequestContext,
	lib::core::action::{ActionScope, RequestContext},
};

pub trait BusinessActionType<C: RequestContext>: Debug + Eq + PartialEq {
	fn scope() -> ActionScope;
}

pub trait BusinessRequestContext<T: BusinessActionType<Self>>: DescriptiveRequestContext {
	fn action_type(&self) -> &T;
}
