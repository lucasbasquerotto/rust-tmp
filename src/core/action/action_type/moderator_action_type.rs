use super::general_action_type::{ActionScope, ActionType};
use std::{collections::HashMap, iter::FromIterator};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter)]
pub enum ModeratorActionType {
	Test,
	EchoInfo,
	EchoWarn,
	EchoError,
	Web,
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
			ModeratorActionType::Test => 0,
			ModeratorActionType::EchoInfo => 1,
			ModeratorActionType::EchoWarn => 2,
			ModeratorActionType::EchoError => 3,
			ModeratorActionType::Web => 4,
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
