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
pub struct DeleteInput(pub UserId);

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

pub struct Delete;

////////////////////////////////////////////////
///////////////////// IMPL /////////////////////
////////////////////////////////////////////////

#[cfg_attr(test, allow(dead_code))]
async fn select(input: SelectInput) -> Result<SelectOutput, ExternalException> {
	Ok(match input {
		SelectInput::ById(id) => SelectOutput {
			id,
			name: format!("User {id:?}").into(),
			email: format!("user-{id:?}@domain.test").into(),
			encrypted_pass: format!("p4$$w0rd{id:?}").into(),
		},
		SelectInput::First => SelectOutput {
			id: UserId(11),
			name: "User 20".into(),
			email: "user-20@domain.test".into(),
			encrypted_pass: "p4$$w0rd20".into(),
		},
		SelectInput::Last => SelectOutput {
			id: UserId(13),
			name: "User 13".into(),
			email: "user-13@domain.test".into(),
			encrypted_pass: "p4$$w0rd13".into(),
		},
	})
}

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
	impl ExternalAction<super::DeleteInput, ()> for super::Delete {
		async fn run(input: super::DeleteInput) -> Result<(), ExternalException> {
			drop(input);
			todo!()
		}
	}

	#[rocket::async_trait]
	impl ExternalAction<super::SelectInput, super::SelectOutput> for super::Select {
		async fn run(input: super::SelectInput) -> Result<super::SelectOutput, ExternalException> {
			super::select(input).await
		}
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

use crate::{
	core::external::data::external_exception::ExternalException, shared::data::user_data::UserId,
};

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

	impl ExternalTest<super::DeleteInput, ()> for super::Delete {
		fn name() -> Str {
			"delete-user".into()
		}

		fn method() -> MockExternalMethod {
			MockExternalMethod::Delete
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
