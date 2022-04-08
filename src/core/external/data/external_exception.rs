use crate::core::action::{
	data::action_data::{DescriptiveError, ErrorData},
	definition::action::ActionError,
};

#[derive(Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub enum ExternalException {
	Unknown,
}

impl ActionError for ExternalException {
	fn private_error(&self) -> DescriptiveError {
		match self {
			ExternalException::Unknown => DescriptiveError::empty(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			ExternalException::Unknown => None,
		}
	}
}
