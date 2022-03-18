use crate::{
	business::action::{
		action_type::action_type::{BusinessActionType, BusinessRequestContext},
		data::action_data::{BusinessException, ErrorData},
		definition::{
			action_error::BusinessErrorGenerator,
			action_helpers::{ActionLogger, ActionTypeHelper, DescriptiveRequestContext},
			business_action::{ActionInput, ActionOutput},
		},
	},
	lib::core::action::{Action, ActionScope, ActionType, Exception, RequestContext},
};

impl<C: DescriptiveRequestContext, T: BusinessActionType<C>>
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
