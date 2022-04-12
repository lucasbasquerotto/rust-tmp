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
	lib::data::result::AsyncResult,
};
use crate::{
	core::{
		action::{
			data::user_action_data::UserActionInput,
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
	pub id: UserId,
}

impl ActionInput for Input {}

////////////////////////////////////////////////
//////////////////// OUTPUT ////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct ItemOutput {
	pub id: UserId,
	pub name: String,
	pub email: String,
}

impl From<user_dao::SelectOutput> for ItemOutput {
	fn from(data: user_dao::SelectOutput) -> Self {
		let user_dao::SelectOutput {
			id, name, email, ..
		} = data;
		Self { id, name, email }
	}
}

#[derive(Debug, PartialEq)]
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
	UserError(Box<UserActionError>),
	ExternalError(Box<ExternalException>),
}

impl ActionError for Error {
	fn private_error(&self) -> DescriptiveError {
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

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct Action(UserRequestInput<Input>);

impl UserAction<Input, Output, Error> for Action {
	fn action_type() -> UserActionType {
		USER_ACTION_TYPE
	}

	fn new(input: UserActionInput<Input>) -> AsyncResult<Self, Error> {
		Box::pin(async { input.map(Self).map_err(Box::new).map_err(Error::UserError) })
	}

	fn run_inner(self) -> AsyncResult<Output, Error> {
		Box::pin(async {
			let Self(input) = self;
			let Input { id } = input.data;

			let first = user_dao::Select::run(user_dao::SelectInput::First)
				.await
				.map_err(Box::new)
				.map_err(Error::ExternalError)?
				.into();

			let last = user_dao::Select::run(user_dao::SelectInput::Last)
				.await
				.map_err(Box::new)
				.map_err(Error::ExternalError)?
				.into();

			let by_id = user_dao::Select::run(user_dao::SelectInput::ById(id))
				.await
				.map_err(Box::new)
				.map_err(Error::ExternalError)?
				.into();

			let result = Output { first, last, by_id };
			Ok(result)
		})
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use crate::core::action::data::action_data::{ActionContext, RequestInput};
	use crate::core::action::data::user_action_data::tests::UserRequestContextBuilder;
	use crate::core::action::data::user_action_data::UserOutputInfo;
	use crate::core::action::definition::action::Action;
	use crate::core::external::definition::external::tests::ExternalMocker;
	use crate::external::dao::main::user_dao;
	use crate::shared::data::user_data::UserId;
	use crate::tests::test_utils::tests::run_test;

	#[tokio::test]
	async fn test_ok() {
		run_test(|_| async {
			let first = user_dao::SelectOutput {
				id: UserId(11),
				name: "User 20".into(),
				email: "user-20@domain.test".into(),
				encrypted_pass: "p4$$w0rd20".into(),
			};

			let by_id = user_dao::SelectOutput {
				id: UserId(12),
				name: "User 12".into(),
				email: "user-12@domain.test".into(),
				encrypted_pass: "p4$$w0rd12".into(),
			};

			let last = user_dao::SelectOutput {
				id: UserId(13),
				name: "User 13".into(),
				email: "user-13@domain.test".into(),
				encrypted_pass: "p4$$w0rd13".into(),
			};

			let _m_first = user_dao::Select::mock(user_dao::SelectInput::First, first.clone());

			let _m_by_id =
				user_dao::Select::mock(user_dao::SelectInput::ById(by_id.id), by_id.clone());

			let _m_last = user_dao::Select::mock(user_dao::SelectInput::Last, last.clone());

			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: super::USER_ACTION_TYPE,
				context: Some(context.clone()),
			};

			let result = super::Action::run(Ok(RequestInput {
				data: super::Input { id: by_id.id },
				context,
			}))
			.await;

			assert_eq!(
				&result,
				&Ok(UserOutputInfo {
					action_context,
					data: super::Output {
						first: first.into(),
						by_id: by_id.into(),
						last: last.into()
					},
				}),
			);
		})
		.await;
	}
}
