use std::fmt::Debug;

use crate::business::action::data::action_data::{BusinessException, ErrorData};

use super::action_helpers::DescriptiveRequestContext;

fn type_of<T>(_: &T) -> String {
	format!("{}", std::any::type_name::<T>())
		.split("::")
		.last()
		.unwrap_or("")
		.to_string()
}

pub trait BusinessErrorGenerator<T: DescriptiveRequestContext>: Debug {
	fn private_error(&self) -> Option<ErrorData>;

	fn public_error(&self) -> Option<ErrorData>;

	fn get_key(&self) -> String {
		let type_name = type_of(&self);
		let key = format!("{type_name}::{self:?}");
		key
	}

	fn error_msg(&self, msg: String) -> Option<ErrorData> {
		Some(ErrorData {
			key: self.get_key(),
			msg,
			params: None,
			meta: None,
		})
	}

	fn exception(&self, context: &T) -> BusinessException<T> {
		BusinessException {
			context: Some(context.clone()),
			private: self.private_error(),
			public: self.public_error(),
		}
	}
}
