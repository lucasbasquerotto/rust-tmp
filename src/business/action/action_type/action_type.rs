use std::fmt::Debug;

use crate::lib::core::action::{ActionScope, RequestContext};

pub trait BusinessActionType<C: RequestContext>: Debug + Eq + PartialEq {
	fn scope() -> ActionScope;
}
