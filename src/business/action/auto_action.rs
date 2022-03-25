use crate::business::{
	action_type::automatic_action_type::AutomaticActionType,
	data::{
		action_data::{ErrorContext, ErrorData, RequestContext, RequestInput},
		automatic_action_data::{
			AutomaticActionError, AutomaticRequestContext, HookRequestContext,
			InternalRequestContext,
		},
	},
	definition::action::{ActionError, ActionInput, ActionOutput, AutomaticAction},
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
	pub auto: String,
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
}

#[derive(Debug)]
pub struct AutoActionInternal(RequestInput<AutoData, InternalRequestContext>);

#[derive(Debug)]
pub struct AutoActionHook(RequestInput<AutoData, HookRequestContext>);

impl AutomaticAction<AutoData, AutoResult, AutoError> for AutoActionInternal {
	fn action_type() -> AutomaticActionType {
		AutomaticActionType::Auto
	}

	fn new(
		input: Result<RequestInput<AutoData, AutomaticRequestContext>, AutomaticActionError>,
	) -> Result<Self, AutoError> {
		input
			.and_then(|ok_input| ok_input.to_internal(Self::action_type()))
			.map(|ok_input| Self(ok_input))
			.map_err(|err| AutoError::AutomaticError(err))
	}

	fn run_inner(self) -> Result<AutoResult, AutoError> {
		let AutoActionInternal(input) = &self;
		run(input, "internal".to_string())
	}
}

impl AutomaticAction<AutoData, AutoResult, AutoError> for AutoActionHook {
	fn action_type() -> AutomaticActionType {
		AutomaticActionType::Auto
	}

	fn new(
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
		run(input, "hook".to_string())
	}
}

fn run<C: RequestContext>(
	input: &RequestInput<AutoData, C>,
	type_name: String,
) -> Result<AutoResult, AutoError> {
	let AutoData { param1, param2 } = &input.data;
	let result = AutoResult {
		id: 1,
		auto: type_name,
		param1: param1.to_string(),
		param2: param2.clone(),
	};
	Ok(result)
}

#[cfg(test)]
mod tests {
	use super::{AutoActionHook, AutoActionInternal, AutoData, AutoError, AutoResult};
	use crate::business::action_type::automatic_action_type::AutomaticActionType;
	use crate::business::data::action_data::{ErrorContext, RequestInput};
	use crate::business::data::automatic_action_data::tests::{
		automatic_context, AutomaticTestOptions,
	};
	use crate::business::data::automatic_action_data::{AutomaticActionError, AutomaticErrorInput};
	use crate::business::definition::action::Action;
	use crate::tests::test_utils::tests::run_test;

	#[test]
	fn test_internal_error_hook() {
		run_test(|_| {
			let context = automatic_context(AutomaticTestOptions { internal: false });

			let result = AutoActionInternal::run(RequestInput {
				data: AutoData {
					param1: "Param 01 (Error)".to_owned(),
					param2: 1,
				},
				context: context.clone(),
			});

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
			let context = automatic_context(AutomaticTestOptions { internal: true });

			let result = AutoActionInternal::run(RequestInput {
				data: AutoData {
					param1: "Param 01 (Ok)".to_owned(),
					param2: 2,
				},
				context: context.clone(),
			});

			assert!(result.as_ref().is_ok());
			assert_eq!(
				result,
				Ok(AutoResult {
					id: 1,
					auto: "internal".to_string(),
					param1: "Param 01 (Ok)".to_owned(),
					param2: 2,
				}),
			);
		});
	}

	#[test]
	fn test_hook_error_internal() {
		run_test(|_| {
			let context = automatic_context(AutomaticTestOptions { internal: true });

			let result = AutoActionHook::run(RequestInput {
				data: AutoData {
					param1: "Param 01 (Error)".to_owned(),
					param2: 3,
				},
				context: context.clone(),
			});

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
			let context = automatic_context(AutomaticTestOptions { internal: false });

			let result = AutoActionHook::run(RequestInput {
				data: AutoData {
					param1: "Param 01 (Ok)".to_owned(),
					param2: 4,
				},
				context: context.clone(),
			});

			assert!(result.as_ref().is_ok());
			assert_eq!(
				result,
				Ok(AutoResult {
					id: 1,
					auto: "hook".to_string(),
					param1: "Param 01 (Ok)".to_owned(),
					param2: 4,
				}),
			);
		});
	}
}
