use core::fmt;
use std::fmt::Debug;

use crate::core::action::{
	action_type::general_action_type::ActionType,
	data::action_data::{ActionErrorInfo, ErrorData, ErrorInfo, RequestContext, RequestInput},
};
use crate::{
	core::action::definition::{
		action::ActionError,
		action_helpers::{ActionErrorHelper, DescriptiveRequestContext},
	},
	lib::data::str::Str,
};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

impl<I, E, C: RequestContext> From<RequestInput<I, C>> for Result<RequestInput<I, C>, E> {
	fn from(input: RequestInput<I, C>) -> Self {
		Ok(input)
	}
}

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
			"[action({action_scope}::{action_type} - {action_id})]",
			action_scope = T::scope(),
			action_type = ActionTypeWrapper(error_context.action_type),
			action_id = error_context.action_type.id(),
		);
		let private = private_error
			.as_ref()
			.and_then(|error| error.msg.as_ref())
			.map(|private| format!("[private={private}]"))
			.unwrap_or_else(|| "".into());
		let public = self
			.error
			.public_error()
			.map(|data| data.msg)
			.map(|public| format!("[public={public}]"))
			.unwrap_or_else(|| "".into());
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
	use crate::core::action::definition::action_helpers::DescriptiveInfo;
	use crate::core::action::{
		action_type::general_action_type::ActionScope, data::action_data::RequestContext,
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
			let action_id = &self.0.id();

			match action_id {
				1 => None,
				2 => Some(DescriptiveError {
					msg: Some("Private message 02".into()),
					data: Some("Data 02".into()),
					source: Some("Source 02".into()),
				}),
				3 => Some(DescriptiveError::empty()),
				4 => Some(DescriptiveError {
					msg: None,
					data: None,
					source: Some("Source 04".into()),
				}),
				5 => None,
				_ => panic!(),
			}
		}

		fn public_error(&self) -> Option<ErrorData> {
			let action_id = self.0.id();

			match action_id {
				1..=3 => {
					Self::error_msg(format!("Test public error (action_id={action_id})").into())
				}
				4 | 5 => None,
				_ => panic!(),
			}
		}
	}

	impl ActionType for TestActionType {
		fn scope() -> ActionScope {
			ActionScope::Automatic
		}

		fn id(&self) -> u32 {
			let Self(id) = self;
			*id
		}

		fn from_id(id: u32) -> Option<Self> {
			Some(Self(id))
		}
	}

	#[tokio::test]
	async fn test_1_public_error() {
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
	async fn test_2_private_public() {
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

			let action = "[action(Automatic::TestActionType - 2)]";
			let private = "[private=Private message 02]";
			let public = "[public=Test public error (action_id=2)]";
			let context = "[context=My error #02]";
			let data = "[data=Data 02]";
			let source = "[source=Source 02]";

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
	async fn test_3_private_public() {
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

			let action = "[action(Automatic::TestActionType - 3)]";
			let public = "[public=Test public error (action_id=3)]";
			let context = "[context=My error #03]";

			assert_eq!(
				&helper.pop_log(),
				&Some(format!("ERROR - {action} {public} {context}").into())
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_4_private_error() {
		run_test(|helper| async move {
			let action_type = TestActionType(4);
			let context = TestRequestContext("My error #04".into());
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

			assert_eq!(&public_error, &None);

			let action = "[action(Automatic::TestActionType - 4)]";
			let context = "[context=My error #04]";
			let source = "[source=Source 04]";

			assert_eq!(
				&helper.pop_log(),
				&Some(format!("ERROR - {action} {context} {source}").into())
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_5_empty_error() {
		run_test(|helper| async move {
			let action_type = TestActionType(5);
			let context = TestRequestContext("My error #05".into());
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

			assert_eq!(&public_error, &None);

			assert_eq!(&helper.pop_log(), &None);
		})
		.await;
	}
}
