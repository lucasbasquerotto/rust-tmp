use crate::business::{
	action_type::action_type::ActionType,
	data::action_data::{ErrorContext, ErrorData, ErrorInput, RequestContext},
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

impl<T: ActionType, C: DescriptiveRequestContext, E: ActionError<T, C>> ActionErrorHelper<T, C>
	for E
{
	fn default_description(&self) -> String {
		let error_context = self.error_context();
		format!(
			"[action({action_id}: {action_type:?})] {public} [context={context}]",
			action_id = error_context.action_type.id(),
			action_type = error_context.action_type,
			public = self
				.public_error()
				.map(|data| data.msg)
				.unwrap_or("".to_string()),
			context = error_context.context.description()
		)
	}

	fn error_msg(&self, msg: String) -> Option<ErrorData> {
		Some(ErrorData { msg, params: None })
	}

	fn type_of<K>(_: &K) -> String {
		format!("{}", std::any::type_name::<T>())
			.split("::")
			.last()
			.unwrap_or("")
			.to_string()
	}

	fn input(error_context: ErrorContext<T, C>) -> ErrorInput<(), T, C> {
		ErrorInput {
			error_context,
			data: (),
			source: None,
		}
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use crate::business::data::action_data::{ActionScope, ErrorContext, ErrorData};
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
	struct TestActionError(ErrorContext<TestActionType, TestRequestContext>);

	impl ActionError<TestActionType, TestRequestContext> for TestActionError {
		fn error_context(&self) -> &ErrorContext<TestActionType, TestRequestContext> {
			&self.0
		}

		fn public_error(&self) -> Option<ErrorData> {
			let action_id = self.error_context().action_type.id();
			self.error_msg(format!("Test public error (action_id={action_id})"))
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
			let error = TestActionError(ErrorContext {
				action_type: action_type.clone(),
				context: context.clone(),
			});
			let public_error = error.handle();
			assert_eq!(
				public_error,
				Some(ErrorData {
					msg: "Test public error (action_id=1)".to_string(),
					params: None
				})
			);
			assert_eq!(
				helper.pop_log(),
				Some(format!(
					"ERROR - [action({action_id}: {action_type:?})] {public} [context={context}]",
					action_id = 1,
					action_type = action_type.clone(),
					public = "Test public error (action_id=1)".to_string(),
					context = "My error #01".to_string()
				))
			);
		});
	}

	#[test]
	fn test_2() {
		run_test(|helper| {
			let action_type = TestActionType(2);
			let context = TestRequestContext("My error #02".to_string());
			let error = TestActionError(ErrorContext {
				action_type: action_type.clone(),
				context: context.clone(),
			});
			let public_error = error.handle();
			assert_eq!(
				public_error,
				Some(ErrorData {
					msg: "Test public error (action_id=2)".to_string(),
					params: None
				})
			);
			assert_eq!(
				helper.pop_log(),
				Some(format!(
					"ERROR - [action({action_id}: {action_type:?})] {public} [context={context}]",
					action_id = 2,
					action_type = action_type.clone(),
					public = "Test public error (action_id=2)".to_string(),
					context = "My error #02".to_string()
				))
			);
		});
	}
}
