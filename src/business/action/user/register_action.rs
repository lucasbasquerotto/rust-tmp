use crate::core::action::{
	data::user_action_data::UserActionInput,
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
	external::dao::user::register_dao::{register_dao, RegisterDaoData, RegisterDaoResult},
};

////////////////////////////////////////////////
///////////////////// TYPE /////////////////////
////////////////////////////////////////////////

const USER_ACTION_TYPE: UserActionType = UserActionType::Register;

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct RegisterData {
	pub name: String,
	pub email: String,
	pub pass: String,
}

impl ActionInput for RegisterData {}

////////////////////////////////////////////////
//////////////////// OUTPUT ////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct RegisterResult {
	pub id: u64,
	pub name: String,
}

impl ActionOutput for RegisterResult {}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum RegisterError {
	UserError(UserActionError),
}

impl ActionError for RegisterError {
	fn private_error(&self) -> DescriptiveError {
		match self {
			RegisterError::UserError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			RegisterError::UserError(error) => error.public_error(),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct RegisterAction(UserNoAuthRequestInput<RegisterData>);

impl UserAction<RegisterData, RegisterResult, RegisterError> for RegisterAction {
	fn action_type() -> UserActionType {
		USER_ACTION_TYPE
	}

	fn new(input: UserActionInput<RegisterData>) -> Result<Self, RegisterError> {
		input
			.and_then(|ok_input| ok_input.into())
			.map(Self)
			.map_err(RegisterError::UserError)
	}

	fn run_inner(self) -> Result<RegisterResult, RegisterError> {
		let RegisterAction(input) = self;
		let RegisterData { name, email, pass } = input.data;
		let RegisterDaoResult { id } = register_dao(RegisterDaoData {
			name: name.to_string(),
			email,
			pass,
		});
		let result = RegisterResult { id, name };
		Ok(result)
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::USER_ACTION_TYPE;
	use super::{RegisterAction, RegisterData, RegisterError, RegisterResult};
	use crate::core::action::data::action_data::{ActionContext, ActionErrorInfo, RequestInput};
	use crate::core::action::data::user_action_data::tests::UserRequestContextBuilder;
	use crate::core::action::data::user_action_data::UserActionError;
	use crate::core::action::data::user_action_data::UserOutputInfo;
	use crate::core::action::definition::action::Action;
	use crate::external::dao::user::register_dao::{RegisterDaoData, RegisterDaoResult};
	use crate::tests::test_utils::tests::{mock_dao, run_test, MockDaoMethod};

	#[test]
	fn test_error_auth() {
		run_test(|_| {
			let context = UserRequestContextBuilder::build_auth();

			let result = RegisterAction::run(RequestInput {
				data: RegisterData {
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
					error: RegisterError::UserError(UserActionError::Authenticated),
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

			let dao_input = RegisterDaoData {
				name: name.into(),
				email: email.into(),
				pass: pass.into(),
			};
			let dao_result = RegisterDaoResult { id };

			let _m = mock_dao(
				"register".into(),
				MockDaoMethod::Insert,
				Some(dao_input),
				Some(dao_result),
			);

			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: USER_ACTION_TYPE,
				context: context.clone(),
			};

			let result = RegisterAction::run(RequestInput {
				data: RegisterData {
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
					data: RegisterResult {
						id,
						name: name.into(),
					},
				}),
			);
		});
	}
}
