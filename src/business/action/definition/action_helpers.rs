use std::fmt::Debug;

use crate::{
	business::action::{
		action_type::action_type::{BusinessActionType, BusinessRequestContext},
		data::{
			action_data::{BusinessException, ErrorData},
			user_action_data::UserRequestContext,
		},
	},
	lib::core::action::ActionType,
};

use super::business_action::{ActionInput, ActionOutput};

pub trait DescriptiveRequestContext: Debug + Clone {
	fn description(&self) -> String;
}

pub trait ActionLogger {
	fn info(&self);
	fn warn(&self);
	fn error(&self);
	fn debug(&self);
}

pub trait UserRequestContextLike {
	fn user_context(&self) -> UserRequestContext;
}

pub trait ActionTypeHelper<C, I, O, T>
where
	C: BusinessRequestContext<T>,
	I: ActionInput,
	O: ActionOutput,
	T: ActionType<C, Option<ErrorData>, BusinessException<C>> + BusinessActionType<C>,
{
	fn validate_type(context: &C) -> Result<(), BusinessException<C>>;
}
