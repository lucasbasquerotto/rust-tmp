use core::fmt;
use std::fmt::Debug;

use crate::business::{
	action_type::action_type::ActionType,
	data::action_data::{ActionErrorInfo, ErrorData, ErrorInfo, RequestContext},
	definition::{
		action::ActionError,
		action_helpers::{ActionErrorHelper, DescriptiveRequestContext},
	},
};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

impl<T: DescriptiveRequestContext> RequestContext for T {}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

// impl<D: Debug + Eq + PartialEq, E: Debug> ErrorInfo<D, E> {
// 	pub fn to_descriptive(&self) -> DescriptiveError {
// 		let Self {
// 			data,
// 			source,
// 		} = self;

// 		DescriptiveError {
// 			data: format!("{data:?}"),
// 			source: format!("{source:?}"),
// 		}
// 	}
// }

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
		let error_context = &self.error_context;
		let action = format!(
			"[action({action_scope:?}::{action_type} - {action_id})]",
			action_scope = T::scope(),
			action_type = ActionTypeWrapper(error_context.action_type.clone()),
			action_id = error_context.action_type.id(),
		);
		let private = format!("[private={private:?}]", private = private_error.msg);
		let public = format!(
			"[public={public}]",
			public = self
				.error
				.public_error()
				.map(|data| data.msg)
				.unwrap_or("".to_string())
		);
		let context = format!(
			"[context={context}]",
			context = error_context.context.description(),
		);
		let data = format!("[data={data:?}]", data = private_error.data,);
		let source = format!("[source={source:?}]", source = private_error.source,);
		format!("{action} {private} {public} {context} {data} {source}",)
	}

	fn handle(self) -> Option<ErrorData> {
		error!("{}", self.description());
		self.error.public_error()
	}

	fn type_of<K>(_: &K) -> String {
		format!("{}", std::any::type_name::<T>())
			.split("::")
			.last()
			.unwrap_or("")
			.to_string()
	}
}

////////////////////////////////////////////////
//////////////////// ACTION ////////////////////
////////////////////////////////////////////////

struct ActionTypeWrapper<T: ActionType>(T);

impl<T: ActionType> fmt::Display for ActionTypeWrapper<T> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		let debug = format!("{:?}", self.0);
		let result = debug.split("(").next().unwrap_or(&debug);
		fmt.write_str(result)
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use crate::business::action_impl::action_impl::ActionTypeWrapper;
	use crate::business::data::action_data::{
		ActionErrorInfo, ActionScope, DescriptiveError, ErrorContext, ErrorData,
	};
	use crate::business::definition::action::ActionError;
	use crate::business::definition::action_helpers::ActionErrorHelper;
	use crate::business::{
		action_type::action_type::ActionType, definition::action_helpers::DescriptiveRequestContext,
	};
	use crate::tests::test_utils::tests::run_test;

	#[derive(Debug, Eq, PartialEq, Clone)]
	struct TestRequestContext(String);

	impl DescriptiveRequestContext for TestRequestContext {
		fn description(&self) -> String {
			self.0.to_string()
		}
	}

	#[derive(Debug, Clone, Eq, PartialEq)]
	struct TestActionType(u32);

	#[derive(Debug)]
	struct TestActionError(TestActionType);

	impl ActionError for TestActionError {
		fn private_error(&self) -> DescriptiveError {
			DescriptiveError::empty()
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
			id.clone()
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
			let error = TestActionError(action_type.clone());
			let error_context = &ErrorContext {
				action_type: action_type.clone(),
				context,
			};
			let error_info = ActionErrorInfo {
				error_context: error_context.clone(),
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
				action_type = ActionTypeWrapper(action_type.clone()),
				action_id = action_type.id(),
			);
			let private = format!("[private=None]");
			let public = format!(
				"[public={public}]",
				public = "Test public error (action_id=1)".to_string(),
			);
			let context = format!("[context={context}]", context = "My error #01".to_string());
			let data = format!("[data=None]");
			let source = format!("[source=None]");

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
			let error = TestActionError(action_type.clone());
			let error_context = &ErrorContext {
				action_type: action_type.clone(),
				context,
			};
			let error_info = ActionErrorInfo {
				error_context: error_context.clone(),
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
				action_type = ActionTypeWrapper(action_type.clone()),
				action_id = action_type.id(),
			);
			let private = format!("[private=None]");
			let public = format!(
				"[public={public}]",
				public = "Test public error (action_id=2)".to_string(),
			);
			let context = format!("[context={context}]", context = "My error #02".to_string());
			let data = format!("[data=None]");
			let source = format!("[source=None]");

			assert_eq!(
				helper.pop_log(),
				Some(format!(
					"ERROR - {action} {private} {public} {context} {data} {source}"
				))
			);
		});
	}
}
