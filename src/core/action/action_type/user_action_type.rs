use super::general_action_type::{ActionScope, ActionType};
use std::{collections::HashMap, iter::FromIterator};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter)]
pub enum UserActionType {
	Test,
	Register,
	Login,
	Logout,
	Web,
}

#[ctor::ctor]
static ID_ACTION_MAP: HashMap<u32, UserActionType> =
	HashMap::from_iter(UserActionType::iter().map(|item| (item.id(), item)));

impl ActionType for UserActionType {
	fn scope() -> ActionScope {
		ActionScope::User
	}

	fn id(&self) -> u32 {
		match self {
			UserActionType::Test => 0,
			UserActionType::Register => 1,
			UserActionType::Login => 2,
			UserActionType::Logout => 3,
			UserActionType::Web => 4,
		}
	}

	fn from_id(id: u32) -> Option<Self> {
		ID_ACTION_MAP.get(&id).copied()
	}
}

#[cfg(test)]
mod tests {
	use super::ID_ACTION_MAP;
	use crate::{
		core::action::action_type::general_action_type::tests::test_enum_action_type,
		tests::test_utils::tests::run_test,
	};

	#[test]
	fn main() {
		run_test(|_| {
			test_enum_action_type(&ID_ACTION_MAP);
		});
	}
}
