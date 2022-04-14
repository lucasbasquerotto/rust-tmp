use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum ActionScope {
	User,
	Moderator,
	Automatic,
}

impl Display for ActionScope {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

pub trait ActionType: Clone + Copy + Debug + Eq + PartialEq {
	fn scope() -> ActionScope;
	fn id(&self) -> u32;
	fn from_id(id: u32) -> Option<Self>;
}

#[cfg(test)]
pub mod tests {
	use std::collections::HashMap;
	use std::collections::HashSet;
	use std::iter::FromIterator;

	use strum::IntoEnumIterator;

	use super::ActionType;

	pub fn test_enum_action_type<T: 'static + ActionType + IntoEnumIterator>(
		id_action_map: &HashMap<u32, T>,
	) {
		let unique_ids_count = HashSet::<u32>::from_iter(T::iter().map(|item| item.id())).len();
		assert_eq!(
			&unique_ids_count,
			&T::iter().count(),
			"test if all the types ids are unique"
		);

		assert_eq!(
			&id_action_map.keys().count(),
			&T::iter().count(),
			"test if there is only 1 id for each type"
		);

		let count = T::iter()
			.filter(|item| {
				let mapped_item = T::from_id(item.id());

				mapped_item == Some(item).copied()
			})
			.count();
		assert_eq!(
			&count,
			&T::iter().count(),
			"test if all the types can be retrieved from their ids"
		);
	}
}
