use std::fmt::Debug;

use crate::business::action::{
	action_type::action_type::ActionType,
	data::action_data::{ErrorContext, ErrorData},
};

use super::action_helpers::DescriptiveRequestContext;

pub trait BusinessException<T: ActionType, C: DescriptiveRequestContext>: Debug
where
	Self: Sized,
{
	fn error_context(&self) -> &ErrorContext<T, C>;

	fn public_error(&self) -> Option<ErrorData>;

	fn description(&self) -> String;

	fn default_description(&self) -> String {
		let error_context = self.error_context();
		format!(
			"[action({action_id}: {action_type:?})] {public} [context={context:?}]",
			action_id = error_context.action_type.id(),
			action_type = error_context.action_type,
			public = self
				.public_error()
				.map(|data| data.msg)
				.unwrap_or("".to_string()),
			context = error_context.context.description()
		)
	}

	fn handle(self) -> Option<ErrorData> {
		let _ = &self.error();
		self.public_error()
	}

	fn error_msg(&self, msg: String) -> Option<ErrorData> {
		Some(ErrorData { msg, params: None })
	}

	fn type_of<K>(_: &K) -> String {
		format!("{}", std::any::type_name::<T>())
			.split("::")
			.last()
			.unwrap_or("")
			.to_string()
	}

	fn info(&self) {
		info!("{}", self.description())
	}

	fn warn(&self) {
		warn!("{}", self.description())
	}

	fn error(&self) {
		error!("{}", self.description())
	}

	fn debug(&self) {
		debug!("{}", self.description())
	}
}
