use crate::lib::core::action::RequestContext;

use super::{
	action_data::general_action_data::BusinessException, business_action::DescriptiveRequestContext,
};

impl<T: DescriptiveRequestContext> RequestContext for T {}

pub trait ActionLogger {
	fn info(&self);
	fn warn(&self);
	fn error(&self);
	fn debug(&self);
}

fn create_msg<C: DescriptiveRequestContext>(
	item: &BusinessException<C>,
	msg_type: String,
) -> String {
	let description = match &item.context {
		Some(info) => info.description(),
		None => "".to_string(),
	};
	format!(
		"{msg_type}: {public:?} ({private:?}) [{description}]",
		public = &item.public,
		private = &item.private
	)
}

impl<C: DescriptiveRequestContext> ActionLogger for BusinessException<C> {
	fn info(&self) {
		info!("{}", create_msg(self, "info".to_string()))
	}

	fn warn(&self) {
		warn!("{}", create_msg(self, "warn".to_string()))
	}

	fn error(&self) {
		error!("{}", create_msg(self, "error".to_string()))
	}

	fn debug(&self) {
		debug!("{}", create_msg(self, "debug".to_string()))
	}
}
