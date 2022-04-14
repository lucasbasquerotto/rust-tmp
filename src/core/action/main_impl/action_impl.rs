use core::fmt;
use std::fmt::Debug;

use crate::core::action::{
	action_type::general_action_type::ActionType,
	data::action_data::{ActionErrorInfo, ErrorData, ErrorInfo},
};
use crate::{
	core::action::definition::{
		action::ActionError,
		action_helpers::{ActionErrorHelper, DescriptiveRequestContext},
	},
	lib::data::str::Str,
};

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
	fn description(&self) -> Str {
		let private_error = &self.error.private_error();
		let error_context = &self.action_context;
		let action = format!(
			"[action({action_scope:?}::{action_type} - {action_id})]",
			action_scope = T::scope(),
			action_type = ActionTypeWrapper(error_context.action_type),
			action_id = error_context.action_type.id(),
		);
		let private = private_error
			.as_ref()
			.and_then(|error| error.msg.as_ref())
			.map(|private| format!("[private={private}]"))
			.unwrap_or_else(|| "".into());
		let public = format!(
			"[public={public}]",
			public = self
				.error
				.public_error()
				.map(|data| data.msg)
				.unwrap_or_else(|| "".into())
		);
		let context = error_context
			.context
			.as_ref()
			.map(|context| format!("[context={context}]", context = context.description()))
			.unwrap_or_else(|| "".into());
		let data = private_error
			.as_ref()
			.and_then(|error| error.data.as_ref())
			.map(|data| format!("[data={data}]"))
			.unwrap_or_else(|| "".into());
		let source = private_error
			.as_ref()
			.and_then(|error| error.source.as_ref())
			.map(|source| format!("[source={source}]"))
			.unwrap_or_else(|| "".into());

		[action, private, public, context, data, source]
			.into_iter()
			.filter(|str| !str.is_empty())
			.collect::<Vec<String>>()
			.join(" ")
			.into()
	}

	fn handle(self) -> Option<ErrorData> {
		if self.error.private_error().is_some() {
			error!("{}", self.description());
		}

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
	use std::borrow::Cow;

	use crate::core::action::action_type::general_action_type::ActionType;
	use crate::core::action::data::action_data::{
		ActionContext, ActionErrorInfo, DescriptiveError, ErrorData,
	};
	use crate::core::action::definition::action::ActionError;
	use crate::core::action::definition::action_helpers::ActionErrorHelper;
	use crate::core::action::{
		action_type::general_action_type::ActionScope, data::action_data::RequestContext,
	};
	use crate::core::action::{
		definition::action_helpers::DescriptiveInfo, main_impl::action_impl::ActionTypeWrapper,
	};
	use crate::lib::data::str::Str;
	use crate::tests::test_utils::tests::run_test;

	#[derive(Debug, Eq, PartialEq, Clone)]
	struct TestRequestContext(Str);

	impl RequestContext for TestRequestContext {}

	impl DescriptiveInfo for TestRequestContext {
		fn description(&self) -> Cow<'_, str> {
			Cow::Borrowed(&self.0)
		}
	}

	#[derive(Debug, Copy, Clone, Eq, PartialEq)]
	struct TestActionType(u32);

	#[derive(Debug)]
	struct TestActionError(TestActionType);

	impl ActionError for TestActionError {
		fn private_error(&self) -> Option<DescriptiveError> {
			let action_type = &self.0;

			if action_type.0 == 1 {
				None
			} else if action_type.0 == 2 {
				Some(DescriptiveError {
					msg: Some("Private message 02".into()),
					data: Some("Data 02".into()),
					source: Some("Source 02".into()),
				})
			} else {
				Some(DescriptiveError::empty())
			}
		}

		fn public_error(&self) -> Option<ErrorData> {
			let action_id = self.0.id();
			Self::error_msg(format!("Test public error (action_id={action_id})").into())
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

	#[tokio::test]
	async fn test_1() {
		run_test(|helper| async move {
			let action_type = TestActionType(1);
			let context = TestRequestContext("My error #01".into());
			let error = TestActionError(action_type);
			let error_context = ActionContext {
				action_type,
				context: Some(context),
			};
			let error_info = ActionErrorInfo {
				action_context: error_context,
				error,
			};
			let public_error = error_info.handle();
			assert_eq!(
				&public_error,
				&Some(ErrorData {
					msg: "Test public error (action_id=1)".into(),
					params: None
				})
			);

			assert_eq!(&helper.pop_log(), &None);
		})
		.await;
	}

	#[tokio::test]
	async fn test_2() {
		run_test(|helper| async move {
			let action_type = TestActionType(2);
			let context = TestRequestContext("My error #02".into());
			let error = TestActionError(action_type);
			let error_context = ActionContext {
				action_type,
				context: Some(context),
			};
			let error_info = ActionErrorInfo {
				action_context: error_context,
				error,
			};
			let public_error = error_info.handle();
			assert_eq!(
				&public_error,
				&Some(ErrorData {
					msg: "Test public error (action_id=2)".into(),
					params: None
				})
			);

			let action = format!(
				"[action({action_scope:?}::{action_type} - {action_id})]",
				action_scope = TestActionType::scope(),
				action_type = ActionTypeWrapper(action_type),
				action_id = action_type.id(),
			);
			let private = format!("[private={private}]", private = "Private message 02");
			let public = format!(
				"[public={public}]",
				public = "Test public error (action_id=2)",
			);
			let context = format!("[context={context}]", context = "My error #02");
			let data = format!("[data={data}]", data = "Data 02");
			let source = format!("[source={source}]", source = "Source 02");

			assert_eq!(
				&helper.pop_log(),
				&Some(
					format!("ERROR - {action} {private} {public} {context} {data} {source}").into()
				)
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_3() {
		run_test(|helper| async move {
			let action_type = TestActionType(3);
			let context = TestRequestContext("My error #03".into());
			let error = TestActionError(action_type);
			let error_context = ActionContext {
				action_type,
				context: Some(context),
			};
			let error_info = ActionErrorInfo {
				action_context: error_context,
				error,
			};
			let public_error = error_info.handle();
			assert_eq!(
				&public_error,
				&Some(ErrorData {
					msg: "Test public error (action_id=3)".into(),
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
				public = "Test public error (action_id=3)",
			);
			let context = format!("[context={context}]", context = "My error #03");

			assert_eq!(
				&helper.pop_log(),
				&Some(format!("ERROR - {action} {public} {context}").into())
			);
		})
		.await;
	}
}
