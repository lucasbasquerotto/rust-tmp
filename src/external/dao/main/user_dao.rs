////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InsertInput {
	pub name: String,
	pub email: String,
	pub pass: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SelectInput {
	ById(UserId),
	First,
	Last,
}

////////////////////////////////////////////////
//////////////////// OUTPUT ////////////////////
////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InsertOutput {
	pub id: UserId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectOutput {
	pub id: UserId,
	pub name: String,
	pub email: String,
	pub encrypted_pass: String,
}

////////////////////////////////////////////////
/////////////////// ACTIONS ////////////////////
////////////////////////////////////////////////

pub struct Insert;

pub struct Select;

////////////////////////////////////////////////
///////////////////// IMPL /////////////////////
////////////////////////////////////////////////

#[cfg(not(test))]
pub mod main {
	use crate::core::{
		action::definition::action::ActionResult,
		external::{
			data::external_exception::ExternalException, definition::external::ExternalAction,
		},
	};

	impl ExternalAction<super::InsertInput, super::InsertOutput> for super::Insert {
		fn run(input: super::InsertInput) -> ActionResult<super::InsertOutput, ExternalException> {
			drop(input);
			todo!()
		}
	}

	impl ExternalAction<super::SelectInput, super::SelectOutput> for super::Select {
		fn run(input: super::SelectInput) -> ActionResult<super::SelectOutput, ExternalException> {
			drop(input);
			todo!()
		}
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

use crate::shared::data::user_data::UserId;

#[cfg(test)]
pub mod tests {
	use crate::{
		core::external::definition::external::tests::{ExternalTest, MockExternalMethod},
		lib::data::str::Str,
	};

	impl ExternalTest<super::InsertInput, super::InsertOutput> for super::Insert {
		fn name() -> Str {
			"register-user".into()
		}

		fn method() -> MockExternalMethod {
			MockExternalMethod::Insert
		}
	}

	impl ExternalTest<super::SelectInput, super::SelectOutput> for super::Select {
		fn name() -> Str {
			"select-user".into()
		}

		fn method() -> MockExternalMethod {
			MockExternalMethod::Select
		}
	}
}
