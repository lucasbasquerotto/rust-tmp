use super::action_type::ActionType;
use crate::business::data::action_data::ActionScope;
use std::{collections::HashMap, iter::FromIterator};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Debug, Eq, PartialEq, EnumIter)]
pub enum AutomaticActionType {
	Test,
}

#[ctor::ctor]
static ID_ACTION_MAP: HashMap<u32, AutomaticActionType> =
	HashMap::from_iter(AutomaticActionType::iter().map(|item| (item.id(), item)));

impl ActionType for AutomaticActionType {
	fn scope() -> ActionScope {
		ActionScope::Automatic
	}

	fn id(&self) -> u32 {
		match self {
			AutomaticActionType::Test => 0,
		}
	}

	fn from_id(id: u32) -> Option<Self> {
		ID_ACTION_MAP.get(&id).map(|item| item.clone())
	}
}

#[cfg(test)]
mod tests {
	use super::ID_ACTION_MAP;
	use crate::business::action_type::action_type::tests::test_enum_action_type;

	#[test]
	fn main() {
		test_enum_action_type(ID_ACTION_MAP.clone());
	}
}
