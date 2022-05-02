use crate::{
	core::{
		action::definition::action::{ActionError, ActionInput, ActionOutput, UserAction},
		external::data::external_exception::ExternalException,
	},
	shared::data::user_data::UserId,
};
use crate::{
	core::{
		action::{
			action_type::user_action_type::UserActionType,
			data::{
				action_data::{DescriptiveError, ErrorData},
				user_action_data::{UserActionError, UserRequestInput},
			},
		},
		external::definition::external::ExternalAction,
	},
	external::dao::main::user_dao,
};

////////////////////////////////////////////////
///////////////////// TYPE /////////////////////
////////////////////////////////////////////////

const USER_ACTION_TYPE: UserActionType = UserActionType::Register;

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct Input(pub UserId);

impl ActionInput for Input {}

////////////////////////////////////////////////
//////////////////// OUTPUT ////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemOutput {
	pub id: UserId,
	pub name: String,
	pub email: String,
}

impl From<user_dao::SelectOutput> for ItemOutput {
	fn from(data: user_dao::SelectOutput) -> Self {
		let user_dao::SelectOutput(user_dao::User {
			id, name, email, ..
		}) = data;
		Self {
			id: UserId(id),
			name,
			email,
		}
	}
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
	pub first: ItemOutput,
	pub last: ItemOutput,
	pub by_id: ItemOutput,
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
pub struct Action(UserRequestInput<Input>);

#[rocket::async_trait]
impl UserAction<Input, Output, Error> for Action {
	fn action_type() -> UserActionType {
		USER_ACTION_TYPE
	}

	async fn new(input: UserRequestInput<Input>) -> Result<Self, Error> {
		Ok(Self(input))
	}

	async fn run_inner(self) -> Result<Output, Error> {
		let Self(input) = self;
		let Input(id) = input.data;

		let first = user_dao::Select::run(user_dao::SelectInput::First)
			.await?
			.into();

		let last = user_dao::Select::run(user_dao::SelectInput::Last)
			.await?
			.into();

		let by_id = user_dao::Select::run(user_dao::SelectInput::ById(id))
			.await?
			.into();

		let result = Output { first, last, by_id };
		Ok(result)
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use mockito::Mock;

	use crate::core::action::data::action_data::{ActionContext, RequestInput};
	use crate::core::action::data::user_action_data::tests::UserRequestContextBuilder;
	use crate::core::action::data::user_action_data::UserOutputInfo;
	use crate::core::action::definition::action::Action;
	use crate::core::external::definition::external::tests::ExternalMocker;
	use crate::external::dao::main::user_dao;
	use crate::shared::data::user_data::UserId;
	use crate::tests::test_utils::tests::run_test;

	pub struct ActionMock {
		pub user_id: UserId,
		pub output: super::Output,
		pub mocks: Vec<Mock>,
	}

	pub fn mock_action(user_id: UserId) -> ActionMock {
		let UserId(user_id) = user_id;

		let first = user_dao::SelectOutput(user_dao::User {
			id: 11,
			name: "User 20".into(),
			email: "user-20@domain.test".into(),
			encrypted_pass: "p4$$w0rd20".into(),
		});

		let by_id = user_dao::SelectOutput(user_dao::User {
			id: user_id,
			name: format!("User {user_id}").into(),
			email: format!("user-{user_id}@domain.test").into(),
			encrypted_pass: format!("p4$$w0rd{user_id}").into(),
		});

		let last = user_dao::SelectOutput(user_dao::User {
			id: 13,
			name: "User 13".into(),
			email: "user-13@domain.test".into(),
			encrypted_pass: "p4$$w0rd13".into(),
		});

		let output = super::Output {
			first: first.clone().into(),
			by_id: by_id.clone().into(),
			last: last.clone().into(),
		};

		let mocks = vec![
			user_dao::Select::mock(user_dao::SelectInput::First, first),
			user_dao::Select::mock(user_dao::SelectInput::ById(UserId(user_id)), by_id),
			user_dao::Select::mock(user_dao::SelectInput::Last, last),
		];

		ActionMock {
			user_id: UserId(user_id),
			output,
			mocks,
		}
	}

	#[tokio::test]
	async fn test_ok() {
		run_test(|_| async {
			let ActionMock {
				user_id,
				output,
				mocks: _m,
			} = mock_action(UserId(12));

			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: super::USER_ACTION_TYPE,
				context: Some(context.clone()),
			};

			let result = super::Action::run(Ok(RequestInput {
				data: super::Input(user_id),
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
