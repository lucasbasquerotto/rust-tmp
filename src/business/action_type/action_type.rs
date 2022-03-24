use std::fmt::Debug;

use crate::business::data::action_data::ActionScope;

pub trait ActionType: Clone + Debug + Eq + PartialEq {
	fn scope() -> ActionScope;
	fn id(&self) -> u32;
	fn from_id(id: u32) -> Option<&'static Self>;
}

#[cfg(test)]
pub mod tests {
	use std::collections::HashMap;
	use std::collections::HashSet;
	use std::iter::FromIterator;

	use strum::IntoEnumIterator;

	use crate::business::action_type::action_type::ActionType;

	pub fn test_enum_action_type<T: 'static + ActionType + IntoEnumIterator>(
		id_action_map: HashMap<u32, T>,
	) {
		let unique_ids_count = HashSet::<u32>::from_iter(T::iter().map(|item| item.id())).len();
		assert_eq!(
			unique_ids_count,
			T::iter().count(),
			"test if all the types ids are unique"
		);

		assert_eq!(
			id_action_map.keys().count(),
			T::iter().count(),
			"test if there is only 1 id for each type"
		);

		let count = T::iter()
			.filter(|item| {
				let mapped_item = T::from_id(item.id());
				let valid = mapped_item == Some(&item);
				valid
			})
			.count();
		assert_eq!(
			count,
			T::iter().count(),
			"test if all the types can be retrieved from their ids"
		);
	}
}
