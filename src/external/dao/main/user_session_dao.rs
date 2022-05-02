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
	use crate::core::external::{
		data::external_exception::ExternalException, definition::external::ExternalAction,
	};

	#[rocket::async_trait]
	impl ExternalAction<super::InsertInput, super::InsertOutput> for super::Insert {
		async fn run(input: super::InsertInput) -> Result<super::InsertOutput, ExternalException> {
			drop(input);
			todo!()
		}
	}

	#[rocket::async_trait]
	impl ExternalAction<super::SelectInput, super::SelectOutput> for super::Select {
		async fn run(input: super::SelectInput) -> Result<super::SelectOutput, ExternalException> {
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
			"register-user-session".into()
		}

		fn method() -> MockExternalMethod {
			MockExternalMethod::Insert
		}
	}

	impl ExternalTest<super::SelectInput, super::SelectOutput> for super::Select {
		fn name() -> Str {
			"select-user-session".into()
		}

		fn method() -> MockExternalMethod {
			MockExternalMethod::Select
		}
	}
}
