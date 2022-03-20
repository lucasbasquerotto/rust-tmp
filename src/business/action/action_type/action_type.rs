use std::fmt::Debug;

use crate::lib::core::action::ActionScope;

pub trait BusinessActionType: Clone + Debug + Eq + PartialEq {
	fn scope() -> ActionScope;
	fn id(&self) -> u32;
}
