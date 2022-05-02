use crate::{
	core::{
		action::definition::action::{ActionError, ActionInput, UserAction},
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

impl From<Input> for user_dao::DeleteInput {
	fn from(input: Input) -> Self {
		user_dao::DeleteInput(input.0)
	}
}

impl ActionInput for Input {}

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
impl UserAction<Input, (), Error> for Action {
	fn action_type() -> UserActionType {
		USER_ACTION_TYPE
	}

	async fn new(input: UserRequestInput<Input>) -> Result<Self, Error> {
		Ok(Self(input))
	}

	async fn run_inner(self) -> Result<(), Error> {
		let Self(input) = self;
		user_dao::Delete::run(input.data.into())
			.await
			.map_err(Error::from)
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
		pub mocks: Vec<Mock>,
	}

	pub fn mock_action(user_id: UserId) -> ActionMock {
		let mocks = vec![user_dao::Delete::mock(user_dao::DeleteInput(user_id), ())];
		ActionMock { user_id, mocks }
	}

	#[tokio::test]
	async fn test_ok() {
		run_test(|_| async {
			let ActionMock { user_id, mocks: _m } = mock_action(UserId(12));

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
					data: (),
				}),
			);
		})
		.await;
	}
}
