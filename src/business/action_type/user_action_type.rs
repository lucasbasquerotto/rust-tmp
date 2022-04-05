use super::general_action_type::{ActionScope, ActionType};
use std::{collections::HashMap, iter::FromIterator};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter)]
pub enum UserActionType {
	Test,
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
			UserActionType::Login => 1,
			UserActionType::Logout => 2,
			UserActionType::Web => 3,
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
		business::action_type::general_action_type::tests::test_enum_action_type,
		tests::test_utils::tests::run_test,
	};

	#[test]
	fn main() {
		run_test(|_| {
			test_enum_action_type(&ID_ACTION_MAP);
		});
	}
}
