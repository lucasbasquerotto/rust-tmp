use std::fmt::Debug;

use crate::{
	business::action::{
		action_type::action_type::{BusinessActionType, BusinessRequestContext},
		data::{
			action_data::{BusinessException, ErrorData},
			user_action_data::UserRequestContext,
		},
	},
	lib::core::action::{Action, ActionType},
};

use super::{
	action_error::BusinessErrorGenerator,
	business_action::{ActionInput, ActionOutput},
};

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

#[derive(Debug)]
struct ExpectedFoundErrorParam {
	expected: String,
	found: String,
}

#[derive(Debug)]
enum ActionHelperError {
	WrongAction(ExpectedFoundErrorParam),
}

impl<C: DescriptiveRequestContext> BusinessErrorGenerator<C> for ActionHelperError {
	fn private_error(&self) -> Option<ErrorData> {
		match self {
			ActionHelperError::WrongAction(ExpectedFoundErrorParam { expected, found }) => {
				BusinessErrorGenerator::<C>::error_msg(
					self,
					format!("Wrong action defined: expected={expected}, found={found}."),
				)
			}
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			ActionHelperError::WrongAction(_) => None,
		}
	}
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

impl<C, I, O, T, A> ActionTypeHelper<C, I, O, T> for A
where
	C: BusinessRequestContext<T>,
	I: ActionInput,
	O: ActionOutput,
	T: ActionType<C, Option<ErrorData>, BusinessException<C>> + BusinessActionType<C>,
	A: Action<C, I, O, Option<ErrorData>, BusinessException<C>, T>,
{
	fn validate_type(context: &C) -> Result<(), BusinessException<C>> {
		let expected = &Self::action_type();
		let found = context.action_type();

		if expected != found {
			Err(ActionHelperError::WrongAction(ExpectedFoundErrorParam {
				expected: format!("{expected:?}").to_string(),
				found: format!("{found:?}").to_string(),
			})
			.exception(context))?;
		}

		Ok(())
	}
}
