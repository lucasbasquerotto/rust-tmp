use crate::core::action::{
	action_type::user_action_type::UserActionType,
	data::{
		action_data::{DescriptiveError, ErrorData},
		user_action_data::{UserActionError, UserNoAuthRequestInput},
	},
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
pub struct LoginData {
	pub name: String,
	pub pass: String,
}

impl ActionInput for LoginData {}

////////////////////////////////////////////////
//////////////////// OUTPUT ////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub struct LoginResult {
	pub id: u64,
	pub name: String,
}

impl ActionOutput for LoginResult {}

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum LoginError {
	UserError(UserActionError),
}

impl ActionError for LoginError {
	fn private_error(&self) -> DescriptiveError {
		match self {
			LoginError::UserError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			LoginError::UserError(error) => error.public_error(),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct LoginAction(UserNoAuthRequestInput<LoginData>);

impl UserAction<LoginData, LoginResult, LoginError> for LoginAction {
	fn action_type() -> UserActionType {
		USER_ACTION_TYPE
	}

	fn new(input: UserActionInput<LoginData>) -> Result<Self, LoginError> {
		input
			.and_then(|ok_input| ok_input.into())
			.map(Self)
			.map_err(LoginError::UserError)
	}

	fn run_inner(self) -> Result<LoginResult, LoginError> {
		let LoginAction(input) = &self;
		let LoginData { name, pass } = &input.data;
		println!("TODO: login: {name} ({pass})");
		let result = LoginResult {
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
	use super::{LoginAction, LoginData, LoginError, LoginResult};
	use crate::business::action::login_action::USER_ACTION_TYPE;
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

			let result = LoginAction::run(RequestInput {
				data: LoginData {
					name: "User 01".to_owned(),
					pass: "p4$$w0rd".to_owned(),
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
					error: LoginError::UserError(UserActionError::Authenticated),
				}),
			);
		});
	}

	#[test]
	fn test_ok() {
		run_test(|_| {
			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: USER_ACTION_TYPE,
				context: context.clone(),
			};

			let result = LoginAction::run(RequestInput {
				data: LoginData {
					name: "User 02".to_owned(),
					pass: "p4$$w0rd2".to_owned(),
				},
				context,
			});

			assert_eq!(
				&result,
				&Ok(UserOutputInfo {
					action_context,
					data: LoginResult {
						id: 1,
						name: "User 02".into(),
					},
				}),
			);
		});
	}
}
