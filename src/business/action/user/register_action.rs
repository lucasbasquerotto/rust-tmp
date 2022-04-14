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
	lib::data::result::AsyncResult,
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

#[derive(Debug, PartialEq)]
pub struct Input {
	pub name: String,
	pub email: String,
	pub pass: String,
}

impl ActionInput for Input {}

////////////////////////////////////////////////
//////////////////// OUTPUT ////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
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

impl UserAction<Input, Output, Error> for Action {
	fn action_type() -> UserActionType {
		USER_ACTION_TYPE
	}

	fn new(input: UserRequestInput<Input>) -> AsyncResult<Self, Error> {
		Box::pin(async {
			UserNoAuthInputResult::from(input)
				.map(Self)
				.map_err(Error::from)
		})
	}

	fn run_inner(self) -> AsyncResult<Output, Error> {
		Box::pin(async {
			let Self(input) = self;
			let Input { name, email, pass } = input.data;
			let user_dao::InsertOutput { id } = user_dao::Insert::run(user_dao::InsertInput {
				name: name.to_string(),
				email,
				pass,
			})
			.await?;
			let result = Output { id, name };
			Ok(result)
		})
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
	use crate::core::external::definition::external::tests::ExternalMocker;
	use crate::external::dao::main::user_dao;
	use crate::shared::data::user_data::UserId;
	use crate::tests::test_utils::tests::run_test;

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
			let id = UserId(7);

			let dao_input = user_dao::InsertInput {
				name: name.into(),
				email: email.into(),
				pass: pass.into(),
			};
			let dao_result = user_dao::InsertOutput { id };

			let _m = user_dao::Insert::mock(dao_input, dao_result);

			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: super::USER_ACTION_TYPE,
				context: Some(context.clone()),
			};

			let result = super::Action::run(Ok(RequestInput {
				data: super::Input {
					name: name.into(),
					email: email.into(),
					pass: pass.into(),
				},
				context,
			}))
			.await;

			assert_eq!(
				&result,
				&Ok(UserOutputInfo {
					action_context,
					data: super::Output {
						id,
						name: name.into(),
					},
				}),
			);
		})
		.await;
	}
}
