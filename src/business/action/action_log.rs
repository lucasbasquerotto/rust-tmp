use crate::lib::core::action_core::RequestInfo;

use super::business_action::BusinessException;

pub trait RequestInfoDescription: RequestInfo {
	fn description(&self) -> String;
}

pub trait ActionLogger {
	fn info(&self);
	fn warn(&self);
	fn error(&self);
	fn debug(&self);
}

fn create_msg<T: RequestInfoDescription>(item: &BusinessException<T>, msg_type: String) -> String {
	let description = match &item.info {
		Some(info) => info.description(),
		None => "".to_string(),
	};
	format!(
		"{msg_type}: {public:?} ({private:?}) [{description}]",
		public = &item.public,
		private = &item.private
	)
}

impl<T: RequestInfoDescription> ActionLogger for BusinessException<T> {
	fn info(&self) {
		info!("{}", create_msg(self, "info".to_string()))
	}

	fn warn(&self) {
		info!("{}", create_msg(self, "warn".to_string()))
	}

	fn error(&self) {
		info!("{}", create_msg(self, "error".to_string()))
	}

	fn debug(&self) {
		info!("{}", create_msg(self, "debug".to_string()))
	}
}
