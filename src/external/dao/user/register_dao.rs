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

#[cfg(not(test))]
pub fn register_dao(data: RegisterDaoData) -> RegisterDaoResult {
	drop(data);
	todo!()
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub fn register_dao(data: RegisterDaoData) -> RegisterDaoResult {
	use crate::tests::test_utils::tests::{test_dao, MockDaoMethod};
	test_dao("register".into(), MockDaoMethod::Insert, Some(data)).unwrap()
}
