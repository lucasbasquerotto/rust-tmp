use crate::core::action::{
	data::user_action_data::{UserNoAuthInputResult, UserRequestInput},
	definition::action::{ActionError, ActionInput, ActionOutput, UserAction},
};
use crate::{
	core::action::{
		action_type::user_action_type::UserActionType,
		data::{
			action_data::{DescriptiveError, ErrorData},
			user_action_data::{UserActionError, UserNoAuthRequestInput},
		},
	},
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
	fn private_error(&self) -> Option<DescriptiveError> {
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

impl From<UserActionError> for Error {
	fn from(error: UserActionError) -> Self {
		Self::UserError(error)
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct Action(UserNoAuthRequestInput<Input>);

#[rocket::async_trait]
impl UserAction<Input, Output, Error> for Action {
	fn action_type() -> UserActionType {
		USER_ACTION_TYPE
	}

	async fn new(input: UserRequestInput<Input>) -> Result<Self, Error> {
		UserNoAuthInputResult::from(input)
			.map(Self)
			.map_err(Error::from)
	}

	async fn run_inner(self) -> Result<Output, Error> {
		let Self(input) = &self;
		let Input { name, pass } = &input.data;
		println!("TODO: login: {name} ({pass})");
		let result = Output {
			id: 1,
			name: name.into(),
		};
		Ok(result)
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use crate::core::action::data::action_data::{ActionContext, ActionErrorInfo, RequestInput};
	use crate::core::action::data::user_action_data::tests::UserRequestContextBuilder;
	use crate::core::action::data::user_action_data::UserActionError;
	use crate::core::action::data::user_action_data::UserOutputInfo;
	use crate::core::action::definition::action::Action;
	use crate::tests::test_utils::tests::run_test;

	#[tokio::test]
	async fn test_error_auth() {
		run_test(|_| async {
			let context = UserRequestContextBuilder::build_auth();

			let result = super::Action::run(Ok(RequestInput {
				data: super::Input {
					name: "User 01".into(),
					pass: "p4$$w0rd".into(),
				},
				context: context.clone(),
			}))
			.await;

			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context: ActionContext {
						action_type: super::USER_ACTION_TYPE,
						context: Some(context),
					},
					error: super::Error::UserError(UserActionError::Authenticated),
				}),
			);
		})
		.await;
	}

	#[tokio::test]
	async fn test_ok() {
		run_test(|_| async {
			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: super::USER_ACTION_TYPE,
				context: Some(context.clone()),
			};

			let result = super::Action::run(Ok(RequestInput {
				data: super::Input {
					name: "User 02".into(),
					pass: "p4$$w0rd2".into(),
				},
				context,
			}))
			.await;

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
		})
		.await;
	}
}
