use crate::core::action::{
	action_type::user_action_type::UserActionType,
	data::{
		action_data::{DescriptiveError, ErrorData},
		user_action_data::{UserActionError, UserNoAuthRequestInput},
	},
	definition::action::ActionResult,
};
use crate::core::action::{
	data::user_action_data::UserActionInput,
	definition::action::{ActionError, ActionInput, ActionOutput, UserAction},
};

////////////////////////////////////////////////
///////////////////// TYPE /////////////////////
////////////////////////////////////////////////

const USER_ACTION_TYPE: UserActionType = UserActionType::Login;

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct Input {
	pub name: String,
	pub pass: String,
}

impl ActionInput for Input {}

////////////////////////////////////////////////
//////////////////// OUTPUT ////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct Output {
	pub id: u64,
	pub name: String,
}

impl ActionOutput for Output {}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum Error {
	UserError(UserActionError),
}

impl ActionError for Error {
	fn private_error(&self) -> DescriptiveError {
		match self {
			Error::UserError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			Error::UserError(error) => error.public_error(),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct Action(UserNoAuthRequestInput<Input>);

impl UserAction<Input, Output, Error> for Action {
	fn action_type() -> UserActionType {
		USER_ACTION_TYPE
	}

	fn new(input: UserActionInput<Input>) -> ActionResult<Self, Error> {
		Box::pin(async move {
			input
				.await
				.and_then(|ok_input| ok_input.into())
				.map(Self)
				.map_err(Error::UserError)
		})
	}

	fn run_inner(self) -> ActionResult<Output, Error> {
		Box::pin(async move {
			let Self(input) = &self;
			let Input { name, pass } = &input.data;
			println!("TODO: login: {name} ({pass})");
			let result = Output {
				id: 1,
				name: name.into(),
			};
			Ok(result)
		})
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use futures::executor::block_on;

	use crate::core::action::data::action_data::{ActionContext, ActionErrorInfo, RequestInput};
	use crate::core::action::data::user_action_data::tests::UserRequestContextBuilder;
	use crate::core::action::data::user_action_data::UserActionError;
	use crate::core::action::data::user_action_data::UserOutputInfo;
	use crate::core::action::definition::action::Action;
	use crate::tests::test_utils::tests::run_test;

	#[test]
	fn test_error_auth() {
		run_test(|_| {
			let context = UserRequestContextBuilder::build_auth();

			let result = block_on(super::Action::run(RequestInput {
				data: super::Input {
					name: "User 01".into(),
					pass: "p4$$w0rd".into(),
				},
				context: context.clone(),
			}));

			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context: ActionContext {
						action_type: super::USER_ACTION_TYPE,
						context,
					},
					error: super::Error::UserError(UserActionError::Authenticated),
				}),
			);
		});
	}

	#[test]
	fn test_ok() {
		run_test(|_| {
			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: super::USER_ACTION_TYPE,
				context: context.clone(),
			};

			let result = block_on(super::Action::run(RequestInput {
				data: super::Input {
					name: "User 02".into(),
					pass: "p4$$w0rd2".into(),
				},
				context,
			}));

			assert_eq!(
				&result,
				&Ok(UserOutputInfo {
					action_context,
					data: super::Output {
						id: 1,
						name: "User 02".into(),
					},
				}),
			);
		});
	}
}
