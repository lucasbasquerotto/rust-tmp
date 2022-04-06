use core::fmt;
use std::fmt::Debug;

use crate::core::action::definition::{
	action::ActionError,
	action_helpers::{ActionErrorHelper, DescriptiveRequestContext},
};
use crate::core::action::{
	action_type::general_action_type::ActionType,
	data::action_data::{ActionErrorInfo, ErrorData, ErrorInfo, RequestContext},
};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

impl<T: DescriptiveRequestContext> RequestContext for T {}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

impl<D: Debug + Eq + PartialEq, E: Debug> PartialEq for ErrorInfo<D, E> {
	fn eq(&self, other: &Self) -> bool {
		self.data == other.data
	}
}

impl<D: Debug + Eq + PartialEq> Eq for ErrorInfo<D> {}

impl<T: ActionType, C: DescriptiveRequestContext, E: ActionError> ActionErrorHelper<T, C, E>
	for ActionErrorInfo<T, C, E>
{
	fn description(&self) -> String {
		let private_error = &self.error.private_error();
		let error_context = &self.action_context;
		let action = format!(
			"[action({action_scope:?}::{action_type} - {action_id})]",
			action_scope = T::scope(),
			action_type = ActionTypeWrapper(error_context.action_type),
			action_id = error_context.action_type.id(),
		);
		let private = private_error
			.msg
			.as_ref()
			.map(|private| format!("[private={private}]"))
			.unwrap_or_else(|| "".to_string());
		let public = format!(
			"[public={public}]",
			public = self
				.error
				.public_error()
				.map(|data| data.msg)
				.unwrap_or_else(|| "".to_string())
		);
		let context = format!(
			"[context={context}]",
			context = error_context.context.description(),
		);
		let data = private_error
			.data
			.as_ref()
			.map(|data| format!("[data={data}]"))
			.unwrap_or_else(|| "".to_string());
		let source = private_error
			.source
			.as_ref()
			.map(|source| format!("[source={source}]"))
			.unwrap_or_else(|| "".to_string());

		[action, private, public, context, data, source]
			.into_iter()
			.filter(|str| !str.is_empty())
			.collect::<Vec<String>>()
			.join(" ")
	}

	fn handle(self) -> Option<ErrorData> {
		error!("{}", self.description());
		self.error.public_error()
	}
}

////////////////////////////////////////////////
//////////////////// ACTION ////////////////////
////////////////////////////////////////////////

struct ActionTypeWrapper<T: ActionType>(T);

impl<T: ActionType> fmt::Display for ActionTypeWrapper<T> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		let debug = format!("{:?}", self.0);
		let result = debug.split('(').next().unwrap_or(&debug);
		fmt.write_str(result)
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use crate::core::action::action_impl::general_action_impl::ActionTypeWrapper;
	use crate::core::action::action_type::general_action_type::ActionScope;
	use crate::core::action::action_type::general_action_type::ActionType;
	use crate::core::action::data::action_data::{
		ActionContext, ActionErrorInfo, DescriptiveError, ErrorData,
	};
	use crate::core::action::definition::action::ActionError;
	use crate::core::action::definition::action_helpers::ActionErrorHelper;
	use crate::core::action::definition::action_helpers::DescriptiveRequestContext;
	use crate::tests::test_utils::tests::run_test;

	#[derive(Debug, Eq, PartialEq, Clone)]
	struct TestRequestContext(String);

	impl DescriptiveRequestContext for TestRequestContext {
		fn description(&self) -> String {
			self.0.to_string()
		}
	}

	#[derive(Debug, Copy, Clone, Eq, PartialEq)]
	struct TestActionType(u32);

	#[derive(Debug)]
	struct TestActionError(TestActionType);

	impl ActionError for TestActionError {
		fn private_error(&self) -> DescriptiveError {
			let action_type = &self.0;

			if action_type.0 == 1 {
				DescriptiveError {
					msg: Some("Private message 01".to_string()),
					data: Some("Data 01".to_string()),
					source: Some("Source 01".to_string()),
				}
			} else {
				DescriptiveError::empty()
			}
		}

		fn public_error(&self) -> Option<ErrorData> {
			let action_id = self.0.id();
			Self::error_msg(format!("Test public error (action_id={action_id})"))
		}
	}

	impl ActionType for TestActionType {
		fn scope() -> ActionScope {
			ActionScope::Automatic
		}

		fn id(&self) -> u32 {
			let TestActionType(id) = self;
			*id
		}

		fn from_id(id: u32) -> Option<Self> {
			Some(TestActionType(id))
		}
	}

	#[test]
	fn test_1() {
		run_test(|helper| {
			let action_type = TestActionType(1);
			let context = TestRequestContext("My error #01".to_string());
			let error = TestActionError(action_type);
			let error_context = ActionContext {
				action_type,
				context,
			};
			let error_info = ActionErrorInfo {
				action_context: error_context,
				error,
			};
			let public_error = error_info.handle();
			assert_eq!(
				public_error,
				Some(ErrorData {
					msg: "Test public error (action_id=1)".to_string(),
					params: None
				})
			);

			let action = format!(
				"[action({action_scope:?}::{action_type} - {action_id})]",
				action_scope = TestActionType::scope(),
				action_type = ActionTypeWrapper(action_type),
				action_id = action_type.id(),
			);
			let private = format!("[private={private}]", private = "Private message 01");
			let public = format!(
				"[public={public}]",
				public = "Test public error (action_id=1)",
			);
			let context = format!("[context={context}]", context = "My error #01");
			let data = format!("[data={data}]", data = "Data 01");
			let source = format!("[source={source}]", source = "Source 01");

			assert_eq!(
				helper.pop_log(),
				Some(format!(
					"ERROR - {action} {private} {public} {context} {data} {source}"
				))
			);
		});
	}

	#[test]
	fn test_2() {
		run_test(|helper| {
			let action_type = TestActionType(2);
			let context = TestRequestContext("My error #02".to_string());
			let error = TestActionError(action_type);
			let error_context = ActionContext {
				action_type,
				context,
			};
			let error_info = ActionErrorInfo {
				action_context: error_context,
				error,
			};
			let public_error = error_info.handle();
			assert_eq!(
				public_error,
				Some(ErrorData {
					msg: "Test public error (action_id=2)".to_string(),
					params: None
				})
			);

			let action = format!(
				"[action({action_scope:?}::{action_type} - {action_id})]",
				action_scope = TestActionType::scope(),
				action_type = ActionTypeWrapper(action_type),
				action_id = action_type.id(),
			);
			let public = format!(
				"[public={public}]",
				public = "Test public error (action_id=2)",
			);
			let context = format!("[context={context}]", context = "My error #02");

			assert_eq!(
				helper.pop_log(),
				Some(format!("ERROR - {action} {public} {context}"))
			);
		});
	}
}
