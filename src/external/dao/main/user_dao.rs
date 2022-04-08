////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterData {
	pub name: String,
	pub email: String,
	pub pass: String,
}

////////////////////////////////////////////////
//////////////////// OUTPUT ////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RegisterResult {
	pub id: u64,
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

pub struct RegisterAction;

#[cfg(not(test))]
pub mod main {
	use super::{RegisterAction, RegisterData, RegisterResult};
	use crate::core::external::{
		data::external_exception::ExternalException, definition::external::ExternalAction,
	};

	impl ExternalAction<RegisterData, RegisterResult> for RegisterAction {
		fn run(input: RegisterData) -> Result<RegisterResult, ExternalException> {
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
	use super::{RegisterAction, RegisterData, RegisterResult};
	use crate::{
		core::external::definition::external::tests::{ExternalTest, MockExternalMethod},
		lib::data::str::Str,
	};

	impl ExternalTest<RegisterData, RegisterResult> for RegisterAction {
		fn name() -> Str {
			"register".into()
		}

		fn method() -> MockExternalMethod {
			MockExternalMethod::Insert
		}
	}
}
