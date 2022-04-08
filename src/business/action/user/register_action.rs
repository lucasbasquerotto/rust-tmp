use crate::core::{
	action::{
		data::user_action_data::UserActionInput,
		definition::action::{ActionError, ActionInput, ActionOutput, UserAction},
	},
	external::data::external_exception::ExternalException,
};
use crate::{
	core::{
		action::{
			action_type::user_action_type::UserActionType,
			data::{
				action_data::{DescriptiveError, ErrorData},
				user_action_data::{UserActionError, UserNoAuthRequestInput},
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
	pub id: u64,
	pub name: String,
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
pub struct RegisterAction(UserNoAuthRequestInput<Input>);

impl UserAction<Input, Output, Error> for RegisterAction {
	fn action_type() -> UserActionType {
		USER_ACTION_TYPE
	}

	fn new(input: UserActionInput<Input>) -> Result<Self, Error> {
		input
			.and_then(|ok_input| ok_input.into())
			.map(Self)
			.map_err(Box::new)
			.map_err(Error::UserError)
	}

	fn run_inner(self) -> Result<Output, Error> {
		let RegisterAction(input) = self;
		let Input { name, email, pass } = input.data;
		let user_dao::RegisterResult { id } =
			user_dao::RegisterAction::run(user_dao::RegisterData {
				name: name.to_string(),
				email,
				pass,
			})
			.map_err(Box::new)
			.map_err(Error::ExternalError)?;
		let result = Output { id, name };
		Ok(result)
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::RegisterAction;
	use super::USER_ACTION_TYPE;
	use crate::core::action::data::action_data::{ActionContext, ActionErrorInfo, RequestInput};
	use crate::core::action::data::user_action_data::tests::UserRequestContextBuilder;
	use crate::core::action::data::user_action_data::UserActionError;
	use crate::core::action::data::user_action_data::UserOutputInfo;
	use crate::core::action::definition::action::Action;
	use crate::core::external::definition::external::tests::ExternalMocker;
	use crate::external::dao::main::user_dao;
	use crate::tests::test_utils::tests::run_test;

	#[test]
	fn test_error_auth() {
		run_test(|_| {
			let context = UserRequestContextBuilder::build_auth();

			let result = RegisterAction::run(RequestInput {
				data: super::Input {
					name: "User 01".into(),
					email: "user-01@domain.test".into(),
					pass: "p4$$w0rd".into(),
				},
				context: context.clone(),
			});

			assert_eq!(
				&result,
				&Err(ActionErrorInfo {
					action_context: ActionContext {
						action_type: USER_ACTION_TYPE,
						context,
					},
					error: super::Error::UserError(Box::new(UserActionError::Authenticated)),
				}),
			);
		});
	}

	#[test]
	fn test_ok() {
		run_test(|_| {
			let name = "User 02";
			let email = "user-02@domain.test";
			let pass = "p4$$w0rd2";
			let id = 7;

			let dao_input = user_dao::RegisterData {
				name: name.into(),
				email: email.into(),
				pass: pass.into(),
			};
			let dao_result = user_dao::RegisterResult { id };

			let _m = user_dao::RegisterAction::mock(dao_input, dao_result);

			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: USER_ACTION_TYPE,
				context: context.clone(),
			};

			let result = RegisterAction::run(RequestInput {
				data: super::Input {
					name: name.into(),
					email: email.into(),
					pass: pass.into(),
				},
				context,
			});

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
		});
	}
}
