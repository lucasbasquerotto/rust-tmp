use crate::core::action::definition::action::{ActionError, ModeratorAction};
use crate::{
	core::action::{
		action_type::moderator_action_type::ModeratorActionType,
		data::{
			action_data::{DescriptiveError, ErrorData},
			moderator_action_data::{ModeratorActionError, ModeratorRequestInput},
		},
	},
	lib::data::result::AsyncResult,
};

////////////////////////////////////////////////
///////////////////// TYPE /////////////////////
////////////////////////////////////////////////

const MODERATOR_ACTION_TYPE: ModeratorActionType = ModeratorActionType::EchoError;

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum Error {
	ModeratorError(ModeratorActionError),
}

impl ActionError for Error {
	fn private_error(&self) -> Option<DescriptiveError> {
		match self {
			Error::ModeratorError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			Error::ModeratorError(error) => error.public_error(),
		}
	}
}

impl From<ModeratorActionError> for Error {
	fn from(error: ModeratorActionError) -> Self {
		Self::ModeratorError(error)
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct Action(ModeratorRequestInput<()>);

impl ModeratorAction<(), (), Error> for Action {
	fn action_type() -> ModeratorActionType {
		MODERATOR_ACTION_TYPE
	}

	fn new(input: ModeratorRequestInput<()>) -> AsyncResult<Self, Error> {
		Box::pin(async { Ok(Self(input)) })
	}

	fn run_inner(self) -> AsyncResult<(), Error> {
		Box::pin(async {
			error!("echo error action");
			Ok(())
		})
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use crate::core::action::data::action_data::RequestInput;
	use crate::core::action::data::action_data::{ActionContext, ActionErrorInfo};
	use crate::core::action::data::moderator_action_data::tests::ModeratorRequestContextBuilder;
	use crate::core::action::data::moderator_action_data::tests::ModeratorSessionBuilder;
	use crate::core::action::data::moderator_action_data::ModeratorActionError;
	use crate::core::action::data::moderator_action_data::ModeratorOutputInfo;
	use crate::core::action::data::moderator_action_data::ModeratorRequestContext;
	use crate::core::action::definition::action::Action;
	use crate::tests::test_utils::tests::run_test;

	fn moderator_context() -> ModeratorRequestContext {
		ModeratorRequestContextBuilder::new()
			.session(
				ModeratorSessionBuilder::new()
					.allowed_actions(vec![super::MODERATOR_ACTION_TYPE])
					.build(),
			)
			.build()
	}

	#[tokio::test]
	async fn test_not_allowed() {
		run_test(|_| async {
			let context = ModeratorRequestContextBuilder::new().build();
			let action_context = ActionContext {
				action_type: super::MODERATOR_ACTION_TYPE,
				context: Some(context.clone()),
			};

			let result = super::Action::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: super::Error::ModeratorError(ModeratorActionError::NotAllowed(
						super::MODERATOR_ACTION_TYPE
					)),
				}),
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_ok() {
		run_test(|helper| async move {
			let context = moderator_context();
			let action_context = ActionContext {
				action_type: super::MODERATOR_ACTION_TYPE,
				context: Some(context.clone()),
			};

			let result = super::Action::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Ok(ModeratorOutputInfo {
					action_context,
					data: (),
				}),
			);
			assert_eq!(&helper.pop_log(), &Some("ERROR - echo error action".into()));
		})
		.await;
	}

	#[tokio::test]
	async fn test_ok_admin() {
		run_test(|helper| async move {
			let context = moderator_context();
			let action_context = ActionContext {
				action_type: super::MODERATOR_ACTION_TYPE,
				context: Some(context.clone()),
			};

			let result = super::Action::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Ok(ModeratorOutputInfo {
					action_context,
					data: (),
				}),
			);
			assert_eq!(&helper.pop_log(), &Some("ERROR - echo error action".into()));
		})
		.await;
	}
}
