use super::general_action_type::{ActionScope, ActionType};
use std::{collections::HashMap, iter::FromIterator};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter)]
pub enum AutomaticActionType {
	Test,
	Auto,
	Web,
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
			AutomaticActionType::Auto => 1,
			AutomaticActionType::Web => 2,
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

	#[tokio::test]
	async fn main() {
		run_test(|_| async {
			test_enum_action_type(&ID_ACTION_MAP);
		})
		.await;
	}
}
