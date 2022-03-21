use std::fmt::Debug;

use crate::business::action::data::action_data::ActionScope;

pub trait ActionType: Clone + Debug + Eq + PartialEq {
	fn scope() -> ActionScope;
	fn id(&self) -> u32;
}
