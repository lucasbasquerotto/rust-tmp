use crate::{
	business::action::{
		action_type::action_type::BusinessActionType,
		data::action_data::{BusinessException, ErrorData},
		definition::action_helpers::{ActionLogger, DescriptiveRequestContext},
	},
	lib::core::action::{ActionScope, ActionType, Exception, RequestContext},
};

impl<C: DescriptiveRequestContext, T: BusinessActionType>
	ActionType<C, Option<ErrorData>, BusinessException<C>> for T
{
	fn scope() -> ActionScope {
		Self::scope()
	}
}

impl<T: DescriptiveRequestContext> RequestContext for T {}

impl<T: DescriptiveRequestContext> Exception<Option<ErrorData>> for BusinessException<T> {
	fn handle(self) -> Option<ErrorData> {
		let _ = &self.error();
		self.public
	}
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
