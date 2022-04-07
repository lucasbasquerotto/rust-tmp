use crate::core::action::{
	action_type::automatic_action_type::AutomaticActionType,
	data::{
		action_data::{DescriptiveError, ErrorData, RequestContext, RequestInput},
		automatic_action_data::{AutomaticActionError, HookRequestInput, InternalRequestInput},
	},
};
use crate::core::action::{
	data::automatic_action_data::AutomaticActionInput,
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
pub struct AutoData {
	pub param1: String,
	pub param2: u64,
}

impl ActionInput for AutoData {}

////////////////////////////////////////////////
//////////////////// OUTPUT ////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct AutoResult {
	pub id: u64,
	pub auto: String,
	pub param1: String,
	pub param2: u64,
}

impl ActionOutput for AutoResult {}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum AutoError {
	AutomaticError(AutomaticActionError),
}

impl ActionError for AutoError {
	fn private_error(&self) -> DescriptiveError {
		match self {
			AutoError::AutomaticError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			AutoError::AutomaticError(error) => error.public_error(),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct AutoActionInternal(InternalRequestInput<AutoData>);

impl AutomaticAction<AutoData, AutoResult, AutoError> for AutoActionInternal {
	fn action_type() -> AutomaticActionType {
		AUTOMATIC_ACTION_TYPE
	}

	fn new(input: AutomaticActionInput<AutoData>) -> Result<Self, AutoError> {
		input
			.and_then(|ok_input| ok_input.into())
			.map(Self)
			.map_err(AutoError::AutomaticError)
	}

	fn run_inner(self) -> Result<AutoResult, AutoError> {
		let AutoActionInternal(input) = self;
		run(input, "internal".into())
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct AutoActionHook(HookRequestInput<AutoData>);

impl AutomaticAction<AutoData, AutoResult, AutoError> for AutoActionHook {
	fn action_type() -> AutomaticActionType {
		AUTOMATIC_ACTION_TYPE
	}

	fn new(input: AutomaticActionInput<AutoData>) -> Result<Self, AutoError> {
		match input {
			Err(err) => Err(AutoError::AutomaticError(err)),
			Ok(ok_input) => {
				let real_input = ok_input.into();

				match real_input {
					Err(err) => Err(AutoError::AutomaticError(err)),
					Ok(real_ok_input) => Ok(Self(real_ok_input)),
				}
			}
		}
	}

	fn run_inner(self) -> Result<AutoResult, AutoError> {
		let AutoActionHook(input) = self;
		run(input, "hook".into())
	}
}

////////////////////////////////////////////////
////////////////// FUNCTIONS ///////////////////
////////////////////////////////////////////////

fn run<C: RequestContext>(
	input: RequestInput<AutoData, C>,
	type_name: String,
) -> Result<AutoResult, AutoError> {
	let AutoData { param1, param2 } = input.data;
	let result = AutoResult {
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
	use super::{AutoActionHook, AutoActionInternal, AutoData, AutoError, AutoResult};
	use crate::business::action::auto_action::AUTOMATIC_ACTION_TYPE;
	use crate::core::action::data::action_data::{ActionContext, ActionErrorInfo, RequestInput};
	use crate::core::action::data::automatic_action_data::tests::AutomaticRequestContextBuilder;
	use crate::core::action::data::automatic_action_data::AutomaticActionError;
	use crate::core::action::data::automatic_action_data::AutomaticOutputInfo;
	use crate::core::action::definition::action::Action;
	use crate::tests::test_utils::tests::run_test;

	#[test]
	fn test_internal_error_hook() {
		run_test(|_| {
			let context = AutomaticRequestContextBuilder::build_hook();
			let action_context = ActionContext {
				action_type: AUTOMATIC_ACTION_TYPE,
				context: context.clone(),
			};

			let result = AutoActionInternal::run(RequestInput {
				data: AutoData {
					param1: "Param 01 (Error)".to_owned(),
					param2: 1,
				},
				context,
			});

			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: AutoError::AutomaticError(AutomaticActionError::NotInternal),
				}),
			);
		});
	}

	#[test]
	fn test_internal_ok() {
		run_test(|_| {
			let context = AutomaticRequestContextBuilder::build_internal();
			let action_context = ActionContext {
				action_type: AUTOMATIC_ACTION_TYPE,
				context: context.clone(),
			};

			let result = AutoActionInternal::run(RequestInput {
				data: AutoData {
					param1: "Param 01 (Ok)".to_owned(),
					param2: 2,
				},
				context,
			});

			assert_eq!(
				&result,
				&Ok(AutomaticOutputInfo {
					action_context,
					data: AutoResult {
						id: 1,
						auto: "internal".into(),
						param1: "Param 01 (Ok)".to_owned(),
						param2: 2,
					},
				})
			);
		});
	}

	#[test]
	fn test_hook_error_internal() {
		run_test(|_| {
			let context = AutomaticRequestContextBuilder::build_internal();
			let action_context = ActionContext {
				action_type: AUTOMATIC_ACTION_TYPE,
				context: context.clone(),
			};

			let result = AutoActionHook::run(RequestInput {
				data: AutoData {
					param1: "Param 01 (Error)".to_owned(),
					param2: 3,
				},
				context,
			});

			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: AutoError::AutomaticError(AutomaticActionError::NotHook),
				}),
			);
		});
	}

	#[test]
	fn test_hook_ok() {
		run_test(|_| {
			let context = AutomaticRequestContextBuilder::build_hook();
			let action_context = ActionContext {
				action_type: AUTOMATIC_ACTION_TYPE,
				context: context.clone(),
			};

			let result = AutoActionHook::run(RequestInput {
				data: AutoData {
					param1: "Param 01 (Ok)".to_owned(),
					param2: 4,
				},
				context,
			});

			assert_eq!(
				&result,
				&Ok(AutomaticOutputInfo {
					action_context,
					data: AutoResult {
						id: 1,
						auto: "hook".into(),
						param1: "Param 01 (Ok)".to_owned(),
						param2: 4,
					},
				})
			);
		});
	}
}
