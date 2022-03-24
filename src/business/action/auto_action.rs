use crate::business::{
	action_type::automatic_action_type::AutomaticActionType,
	data::{
		action_data::{ErrorContext, ErrorData, RequestInput},
		automatic_action_data::{
			AutomaticActionError, AutomaticRequestContext, HookRequestContext,
			InternalRequestContext,
		},
	},
	definition::{
		action::{ActionError, ActionInput, ActionOutput, AutomaticAction},
		action_helpers::ActionErrorHelper,
	},
};

#[derive(Debug, PartialEq)]
pub struct AutoData {
	pub param1: String,
	pub param2: u64,
}

impl ActionInput for AutoData {}

#[derive(Debug, PartialEq)]
pub struct AutoResult {
	pub id: u64,
	pub param1: String,
	pub param2: u64,
}

impl ActionOutput for AutoResult {}

#[derive(Debug, PartialEq)]
pub enum AutoError {
	AutomaticError(AutomaticActionError),
}

impl ActionError<AutomaticActionType, AutomaticRequestContext> for AutoError {
	fn error_context(&self) -> &ErrorContext<AutomaticActionType, AutomaticRequestContext> {
		match &self {
			&Self::AutomaticError(error) => error.error_context(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match &self {
			&Self::AutomaticError(error) => error.public_error(),
		}
	}

	fn description(&self) -> String {
		self.default_description()
	}
}

#[derive(Debug)]
pub struct AutoActionInternal(RequestInput<AutoData, InternalRequestContext>);

#[derive(Debug)]
pub struct AutoActionHook(RequestInput<AutoData, HookRequestContext>);

impl AutomaticAction<AutoData, AutoResult, AutoError> for AutoActionInternal {
	fn action_type() -> AutomaticActionType {
		AutomaticActionType::Auto
	}

	fn new_inner(
		input: Result<RequestInput<AutoData, AutomaticRequestContext>, AutomaticActionError>,
	) -> Result<Self, AutoError> {
		match input {
			Err(err) => Err(AutoError::AutomaticError(err)),
			Ok(ok_input) => {
				let real_input = ok_input.to_internal(Self::action_type());

				match real_input {
					Err(err) => Err(AutoError::AutomaticError(err)),
					Ok(real_ok_input) => Ok(Self(real_ok_input)),
				}
			}
		}
	}

	fn run_inner(self) -> Result<AutoResult, AutoError> {
		let AutoActionInternal(input) = &self;
		let AutoData { param1, param2 } = &input.data;
		println!("auto: {param1} ({param2})");
		let result = AutoResult {
			id: 1,
			param1: param1.to_string(),
			param2: param2.clone(),
		};
		Ok(result)
	}
}

impl AutomaticAction<AutoData, AutoResult, AutoError> for AutoActionHook {
	fn action_type() -> AutomaticActionType {
		AutomaticActionType::Auto
	}

	fn new_inner(
		input: Result<RequestInput<AutoData, AutomaticRequestContext>, AutomaticActionError>,
	) -> Result<Self, AutoError> {
		match input {
			Err(err) => Err(AutoError::AutomaticError(err)),
			Ok(ok_input) => {
				let real_input = ok_input.to_hook(Self::action_type());

				match real_input {
					Err(err) => Err(AutoError::AutomaticError(err)),
					Ok(real_ok_input) => Ok(Self(real_ok_input)),
				}
			}
		}
	}

	fn run_inner(self) -> Result<AutoResult, AutoError> {
		let AutoActionHook(input) = &self;
		let AutoData { param1, param2 } = &input.data;
		println!("auto: {param1} ({param2})");
		let result = AutoResult {
			id: 1,
			param1: param1.to_string(),
			param2: param2.clone(),
		};
		Ok(result)
	}
}

#[cfg(test)]
mod tests {
	use super::{AutoActionHook, AutoActionInternal, AutoData, AutoError, AutoResult};
	use crate::business::action_type::automatic_action_type::AutomaticActionType;
	use crate::business::data::action_data::ErrorContext;
	use crate::business::data::automatic_action_data::{AutomaticActionError, AutomaticErrorInput};
	use crate::tests::test_utils::tests::{
		automatic_context, run_test, AutomaticOptions, TestRequest,
	};

	#[test]
	fn test_internal_error_hook() {
		run_test(|_| {
			let context = automatic_context(AutomaticOptions { internal: false });

			let result = AutoActionInternal::test_request(
				AutoData {
					param1: "Param 01 (Error)".to_owned(),
					param2: 1,
				},
				context.clone(),
			);

			assert_eq!(
				&result,
				&Err(AutoError::AutomaticError(
					AutomaticActionError::NotInternal(AutomaticErrorInput {
						error_context: ErrorContext {
							action_type: AutomaticActionType::Auto,
							context: context.clone(),
						},
						data: (),
					})
				))
			);
		});
	}

	#[test]
	fn test_internal_ok() {
		run_test(|_| {
			let context = automatic_context(AutomaticOptions { internal: true });

			let result = AutoActionInternal::test_request(
				AutoData {
					param1: "Param 01 (Ok)".to_owned(),
					param2: 2,
				},
				context.clone(),
			);

			assert!(result.as_ref().is_ok());
			assert_eq!(
				result,
				Ok(AutoResult {
					id: 1,
					param1: "Param 01 (Ok)".to_owned(),
					param2: 2,
				}),
			);
		});
	}

	#[test]
	fn test_hook_error_internal() {
		run_test(|_| {
			let context = automatic_context(AutomaticOptions { internal: true });

			let result = AutoActionHook::test_request(
				AutoData {
					param1: "Param 01 (Error)".to_owned(),
					param2: 3,
				},
				context.clone(),
			);

			assert_eq!(
				&result,
				&Err(AutoError::AutomaticError(AutomaticActionError::NotHook(
					AutomaticErrorInput {
						error_context: ErrorContext {
							action_type: AutomaticActionType::Auto,
							context: context.clone(),
						},
						data: (),
					}
				)))
			);
		});
	}

	#[test]
	fn test_hook_ok() {
		run_test(|_| {
			let context = automatic_context(AutomaticOptions { internal: false });

			let result = AutoActionHook::test_request(
				AutoData {
					param1: "Param 01 (Ok)".to_owned(),
					param2: 4,
				},
				context.clone(),
			);

			assert!(result.as_ref().is_ok());
			assert_eq!(
				result,
				Ok(AutoResult {
					id: 1,
					param1: "Param 01 (Ok)".to_owned(),
					param2: 4,
				}),
			);
		});
	}
}
