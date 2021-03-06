use crate::core::action::{
	action_type::automatic_action_type::AutomaticActionType,
	data::{
		action_data::{DescriptiveError, ErrorData, RequestContext, RequestInput},
		automatic_action_data::{AutomaticActionError, HookRequestInput, InternalRequestInput},
	},
};
use crate::core::action::{
	data::automatic_action_data::{AutomaticRequestInput, HookInputResult, InternalInputResult},
	definition::action::{ActionError, ActionInput, ActionOutput, AutomaticAction},
};

////////////////////////////////////////////////
///////////////////// TYPE /////////////////////
////////////////////////////////////////////////

const AUTOMATIC_ACTION_TYPE: AutomaticActionType = AutomaticActionType::Auto;

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct Input {
	pub param1: String,
	pub param2: u64,
}

impl ActionInput for Input {}

////////////////////////////////////////////////
//////////////////// OUTPUT ////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct Output {
	pub id: u64,
	pub auto: String,
	pub param1: String,
	pub param2: u64,
}

impl ActionOutput for Output {}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum Error {
	AutomaticError(AutomaticActionError),
}

impl ActionError for Error {
	fn private_error(&self) -> Option<DescriptiveError> {
		match self {
			Error::AutomaticError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			Error::AutomaticError(error) => error.public_error(),
		}
	}
}

impl From<AutomaticActionError> for Error {
	fn from(error: AutomaticActionError) -> Self {
		Self::AutomaticError(error)
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct Internal(InternalRequestInput<Input>);

#[rocket::async_trait]
impl AutomaticAction<Input, Output, Error> for Internal {
	fn action_type() -> AutomaticActionType {
		AUTOMATIC_ACTION_TYPE
	}

	async fn new(input: AutomaticRequestInput<Input>) -> Result<Self, Error> {
		InternalInputResult::from(input)
			.map(Self)
			.map_err(Error::from)
	}

	async fn run_inner(self) -> Result<Output, Error> {
		let Self(input) = self;
		run(input, "internal".into())
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct Hook(HookRequestInput<Input>);

#[rocket::async_trait]
impl AutomaticAction<Input, Output, Error> for Hook {
	fn action_type() -> AutomaticActionType {
		AUTOMATIC_ACTION_TYPE
	}

	async fn new(input: AutomaticRequestInput<Input>) -> Result<Self, Error> {
		HookInputResult::from(input).map(Self).map_err(Error::from)
	}

	async fn run_inner(self) -> Result<Output, Error> {
		let Self(input) = self;
		run(input, "hook".into())
	}
}

////////////////////////////////////////////////
////////////////// FUNCTIONS ///////////////////
////////////////////////////////////////////////

fn run<C: RequestContext>(
	input: RequestInput<Input, C>,
	type_name: String,
) -> Result<Output, Error> {
	let Input { param1, param2 } = input.data;
	let result = Output {
		id: 1,
		auto: type_name,
		param1,
		param2,
	};
	Ok(result)
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use crate::core::action::data::action_data::{ActionContext, ActionErrorInfo, RequestInput};
	use crate::core::action::data::automatic_action_data::tests::AutomaticRequestContextBuilder;
	use crate::core::action::data::automatic_action_data::AutomaticActionError;
	use crate::core::action::data::automatic_action_data::AutomaticOutputInfo;
	use crate::core::action::definition::action::Action;
	use crate::tests::test_utils::tests::run_test;

	#[tokio::test]
	async fn test_internal_error_hook() {
		run_test(|_| async {
			let context = AutomaticRequestContextBuilder::build_hook();
			let action_context = ActionContext {
				action_type: super::AUTOMATIC_ACTION_TYPE,
				context: Some(context.clone()),
			};

			let result = super::Internal::run(Ok(RequestInput {
				data: super::Input {
					param1: "Param 01 (Error)".into(),
					param2: 1,
				},
				context,
			}))
			.await;

			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: super::Error::AutomaticError(AutomaticActionError::NotInternal),
				}),
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_internal_ok() {
		run_test(|_| async {
			let context = AutomaticRequestContextBuilder::build_internal();
			let action_context = ActionContext {
				action_type: super::AUTOMATIC_ACTION_TYPE,
				context: Some(context.clone()),
			};

			let result = super::Internal::run(Ok(RequestInput {
				data: super::Input {
					param1: "Param 01 (Ok)".into(),
					param2: 2,
				},
				context,
			}))
			.await;

			assert_eq!(
				&result,
				&Ok(AutomaticOutputInfo {
					action_context,
					data: super::Output {
						id: 1,
						auto: "internal".into(),
						param1: "Param 01 (Ok)".into(),
						param2: 2,
					},
				})
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_hook_error_internal() {
		run_test(|_| async {
			let context = AutomaticRequestContextBuilder::build_internal();
			let action_context = ActionContext {
				action_type: super::AUTOMATIC_ACTION_TYPE,
				context: Some(context.clone()),
			};

			let result = super::Hook::run(Ok(RequestInput {
				data: super::Input {
					param1: "Param 01 (Error)".into(),
					param2: 3,
				},
				context,
			}))
			.await;

			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: super::Error::AutomaticError(AutomaticActionError::NotHook),
				}),
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_hook_ok() {
		run_test(|_| async {
			let context = AutomaticRequestContextBuilder::build_hook();
			let action_context = ActionContext {
				action_type: super::AUTOMATIC_ACTION_TYPE,
				context: Some(context.clone()),
			};

			let result = super::Hook::run(Ok(RequestInput {
				data: super::Input {
					param1: "Param 01 (Ok)".into(),
					param2: 4,
				},
				context,
			}))
			.await;

			assert_eq!(
				&result,
				&Ok(AutomaticOutputInfo {
					action_context,
					data: super::Output {
						id: 1,
						auto: "hook".into(),
						param1: "Param 01 (Ok)".into(),
						param2: 4,
					},
				})
			);
		})
		.await;
	}
}
