use crate::{
	core::{
		action::{
			action_type::user_action_type::UserActionType,
			data::{
				action_data::{DescriptiveError, ErrorData},
				user_action_data::{UserActionError, UserNoAuthRequestInput, UserRequestInput},
			},
		},
		external::definition::external::ExternalAction,
	},
	external::dao::main::user_dao,
};
use crate::{
	core::{
		action::{
			data::user_action_data::UserNoAuthInputResult,
			definition::action::{ActionError, ActionInput, ActionOutput, UserAction},
		},
		external::data::external_exception::ExternalException,
	},
	shared::data::user_data::UserId,
};

////////////////////////////////////////////////
///////////////////// TYPE /////////////////////
////////////////////////////////////////////////

const USER_ACTION_TYPE: UserActionType = UserActionType::Register;

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Input {
	pub name: String,
	pub email: String,
	pub pass: String,
}

impl ActionInput for Input {}

impl From<Input> for user_dao::InsertInput {
	fn from(input: Input) -> Self {
		let Input { name, email, pass } = input;
		user_dao::InsertInput {
			name: name,
			email,
			pass,
		}
	}
}

////////////////////////////////////////////////
//////////////////// OUTPUT ////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
	pub id: UserId,
	pub name: String,
}

impl ActionOutput for Output {}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum Error {
	UserError(UserActionError),
	ExternalError(ExternalException),
}

impl ActionError for Error {
	fn private_error(&self) -> Option<DescriptiveError> {
		match self {
			Error::UserError(error) => error.private_error(),
			Error::ExternalError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			Error::UserError(error) => error.public_error(),
			Error::ExternalError(error) => error.public_error(),
		}
	}
}

impl From<UserActionError> for Error {
	fn from(error: UserActionError) -> Self {
		Self::UserError(error)
	}
}

impl From<ExternalException> for Error {
	fn from(error: ExternalException) -> Self {
		Self::ExternalError(error)
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
		let Self(input) = self;
		let name = input.data.name.to_string();
		let user_dao::InsertOutput { id } =
			user_dao::Insert::run(user_dao::InsertInput::from(input.data)).await?;
		let result = Output { id, name };
		Ok(result)
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use mockito::Mock;

	use crate::core::action::data::action_data::{ActionContext, ActionErrorInfo, RequestInput};
	use crate::core::action::data::user_action_data::tests::UserRequestContextBuilder;
	use crate::core::action::data::user_action_data::UserActionError;
	use crate::core::action::data::user_action_data::UserOutputInfo;
	use crate::core::action::definition::action::Action;
	use crate::core::external::definition::external::tests::ExternalMocker;
	use crate::external::dao::main::user_dao;
	use crate::shared::data::user_data::UserId;
	use crate::tests::test_utils::tests::run_test;

	pub struct ActionMock {
		pub output: super::Output,
		pub mocks: Vec<Mock>,
	}

	pub fn mock_action(input: super::Input) -> ActionMock {
		let input = user_dao::InsertInput::from(input);
		let user_id = UserId(7);
		let dao_result = user_dao::InsertOutput { id: user_id };

		let output = super::Output {
			id: user_id,
			name: input.name.to_string(),
		};

		let mocks = vec![user_dao::Insert::mock(input, dao_result)];

		ActionMock { output, mocks }
	}

	#[tokio::test]
	async fn test_error_auth() {
		run_test(|_| async {
			let context = UserRequestContextBuilder::build_auth();

			let result = super::Action::run(Ok(RequestInput {
				data: super::Input {
					name: "User 01".into(),
					email: "user-01@domain.test".into(),
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
			let name = "User 02";
			let email = "user-02@domain.test";
			let pass = "p4$$w0rd2";

			let input = super::Input {
				name: name.into(),
				email: email.into(),
				pass: pass.into(),
			};
			let ActionMock { output, mocks: _m } = mock_action(input.clone());

			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: super::USER_ACTION_TYPE,
				context: Some(context.clone()),
			};

			let result = super::Action::run(Ok(RequestInput {
				data: input,
				context,
			}))
			.await;

			assert_eq!(
				&result,
				&Ok(UserOutputInfo {
					action_context,
					data: output,
				}),
			);
		})
		.await;
	}
}
