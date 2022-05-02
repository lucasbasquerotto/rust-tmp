use crate::core::action::{
	data::action_data::{DescriptiveError, ErrorData},
	definition::action::ActionError,
};

#[derive(Debug)]
pub struct DbErrorWrapper(pub diesel::result::Error);

#[derive(Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub enum ExternalException {
	Unknown,
	DbError(DbErrorWrapper),
}

impl PartialEq for DbErrorWrapper {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl Eq for DbErrorWrapper {}

impl From<diesel::result::Error> for ExternalException {
	fn from(error: diesel::result::Error) -> Self {
		ExternalException::DbError(DbErrorWrapper(error))
	}
}

impl ActionError for ExternalException {
	fn private_error(&self) -> Option<DescriptiveError> {
		match self {
			ExternalException::Unknown => Some(DescriptiveError::empty()),
			ExternalException::DbError(DbErrorWrapper(source)) => {
				Some(DescriptiveError::source(source))
			}
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			ExternalException::Unknown => None,
			ExternalException::DbError(_) => None,
		}
	}
}
