use crate::{
	business::action::{
		action_type::action_type::BusinessActionType,
		definition::action_helpers::DescriptiveRequestContext,
	},
	lib::core::action::{ActionScope, ActionType, RequestContext},
};

impl<T: DescriptiveRequestContext> RequestContext for T {}

impl<T: BusinessActionType> ActionType for T {
	fn scope() -> ActionScope {
		Self::scope()
	}
}
