use super::action_type::ActionType;
use crate::business::data::action_data::ActionScope;
use std::{collections::HashMap, iter::FromIterator};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Debug, Eq, PartialEq, EnumIter)]
pub enum ModeratorActionType {
	EchoInfo,
	EchoWarn,
	EchoError,
}

#[ctor::ctor]
static ID_ACTION_MAP: HashMap<u32, ModeratorActionType> =
	HashMap::from_iter(ModeratorActionType::iter().map(|item| (item.id(), item)));

impl ActionType for ModeratorActionType {
	fn scope() -> ActionScope {
		ActionScope::Moderator
	}

	fn id(&self) -> u32 {
		match self {
			ModeratorActionType::EchoInfo => 1,
			ModeratorActionType::EchoWarn => 2,
			ModeratorActionType::EchoError => 3,
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
