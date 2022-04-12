use crate::core::action::{
	action_type::moderator_action_type::ModeratorActionType,
	data::{
		action_data::{DescriptiveError, ErrorData},
		moderator_action_data::{ModeratorActionError, ModeratorRequestInput},
	},
	definition::action::ActionResult,
};
use crate::core::action::{
	data::moderator_action_data::ModeratorActionInput,
	definition::action::{ActionError, ModeratorAction},
};

////////////////////////////////////////////////
///////////////////// TYPE /////////////////////
////////////////////////////////////////////////

const MODERATOR_ACTION_TYPE: ModeratorActionType = ModeratorActionType::EchoWarn;

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum Error {
	ModeratorError(ModeratorActionError),
}

impl ActionError for Error {
	fn private_error(&self) -> DescriptiveError {
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

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct Action(ModeratorRequestInput<()>);

impl ModeratorAction<(), (), Error> for Action {
	fn action_type() -> ModeratorActionType {
		MODERATOR_ACTION_TYPE
	}

	fn new(input: ModeratorActionInput<()>) -> ActionResult<Self, Error> {
		Box::pin(async move {
			match input.await {
				Err(err) => Err(Error::ModeratorError(err)),
				Ok(ok_input) => Ok(Self(ok_input)),
			}
		})
	}

	fn run_inner(self) -> ActionResult<(), Error> {
		Box::pin(async move {
			warn!("echo warn action");
			Ok(())
		})
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use futures::executor::block_on;

	use crate::core::action::data::action_data::ActionContext;
	use crate::core::action::data::action_data::ActionErrorInfo;
	use crate::core::action::data::moderator_action_data::tests::{
		ModeratorRequestContextBuilder, ModeratorSessionBuilder,
	};
	use crate::core::action::data::moderator_action_data::{
		ModeratorActionError, ModeratorRequestContext,
	};
	use crate::core::action::data::{
		action_data::RequestInput, moderator_action_data::ModeratorOutputInfo,
	};
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

	#[test]
	fn test_not_allowed() {
		run_test(|_| {
			let context = ModeratorRequestContextBuilder::new().build();
			let action_context = ActionContext {
				action_type: super::MODERATOR_ACTION_TYPE,
				context: context.clone(),
			};

			let result = block_on(super::Action::run(RequestInput { data: (), context }));
			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context,
					error: super::Error::ModeratorError(ModeratorActionError::NotAllowed(
						super::MODERATOR_ACTION_TYPE
					)),
				}),
			);
		});
	}

	#[test]
	fn test_ok() {
		run_test(|helper| {
			let context = moderator_context();
			let action_context = ActionContext {
				action_type: super::MODERATOR_ACTION_TYPE,
				context: context.clone(),
			};

			let result = block_on(super::Action::run(RequestInput { data: (), context }));
			assert_eq!(
				&result,
				&Ok(ModeratorOutputInfo {
					action_context,
					data: (),
				}),
			);
			assert_eq!(&helper.pop_log(), &Some("WARN - echo warn action".into()));
		});
	}

	#[test]
	fn test_ok_admin() {
		run_test(|helper| {
			let context = moderator_context();
			let action_context = ActionContext {
				action_type: super::MODERATOR_ACTION_TYPE,
				context: context.clone(),
			};

			let result = block_on(super::Action::run(RequestInput { data: (), context }));
			assert_eq!(
				&result,
				&Ok(ModeratorOutputInfo {
					action_context,
					data: (),
				}),
			);
			assert_eq!(&helper.pop_log(), &Some("WARN - echo warn action".into()));
		});
	}
}
