////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterDaoData {
	pub name: String,
	pub email: String,
	pub pass: String,
}

////////////////////////////////////////////////
//////////////////// OUTPUT ////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterDaoResult {
	pub id: u64,
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

pub struct RegisterDaoAction;

#[cfg(not(test))]
pub mod main {
	use super::{RegisterDaoAction, RegisterDaoData, RegisterDaoResult};
	use crate::core::external::{
		data::external_exception::ExternalException, definition::external::ExternalAction,
	};

	impl ExternalAction<RegisterDaoData, RegisterDaoResult> for RegisterDaoAction {
		fn run(input: RegisterDaoData) -> Result<RegisterDaoResult, ExternalException> {
			drop(input);
			todo!()
		}
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use super::{RegisterDaoAction, RegisterDaoData, RegisterDaoResult};
	use crate::{
		core::external::definition::external::tests::{ExternalTest, MockExternalMethod},
		lib::data::str::Str,
	};

	impl ExternalTest<RegisterDaoData, RegisterDaoResult> for RegisterDaoAction {
		fn name() -> Str {
			"register".into()
		}

		fn method() -> MockExternalMethod {
			MockExternalMethod::Insert
		}
	}
}
