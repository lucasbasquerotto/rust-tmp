use crate::business::{
	action_type::user_action_type::UserActionType,
	data::{
		action_data::{ErrorContext, ErrorData, RequestInput},
		user_action_data::{UserActionError, UserNoAuthRequestContext, UserRequestContext},
	},
	definition::action::{ActionError, ActionInput, ActionOutput, UserAction},
};

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

impl ActionError<UserActionType, UserRequestContext> for LoginError {
	fn error_context(&self) -> &ErrorContext<UserActionType, UserRequestContext> {
		match &self {
			&Self::UserError(error) => error.error_context(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match &self {
			&Self::UserError(error) => error.public_error(),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct LoginAction(RequestInput<LoginData, UserNoAuthRequestContext>);

impl UserAction<LoginData, LoginResult, LoginError> for LoginAction {
	fn action_type() -> UserActionType {
		UserActionType::Login
	}

	fn new(
		input: Result<RequestInput<LoginData, UserRequestContext>, UserActionError>,
	) -> Result<Self, LoginError> {
		input
			.and_then(|ok_input| ok_input.to_no_auth(Self::action_type()))
			.map(|ok_input| Self(ok_input))
			.map_err(|err| LoginError::UserError(err))
	}

	fn run_inner(self) -> Result<LoginResult, LoginError> {
		let LoginAction(input) = &self;
		let LoginData { name, pass } = &input.data;
		println!("login: {name} ({pass})");
		let result = LoginResult {
			id: 1,
			name: name.to_string(),
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
	use crate::business::action_type::user_action_type::UserActionType;
	use crate::business::data::action_data::{ErrorContext, RequestInput};
	use crate::business::data::user_action_data::tests::{user_context, UserTestOptions};
	use crate::business::data::user_action_data::UserActionError;
	use crate::business::definition::action::Action;
	use crate::business::definition::action_helpers::ActionErrorHelper;
	use crate::tests::test_utils::tests::run_test;

	#[test]
	fn test_error_auth() {
		run_test(|_| {
			let context = user_context(UserTestOptions { user_id: Some(1) });

			let result = LoginAction::run(RequestInput {
				data: LoginData {
					name: "User 01".to_owned(),
					pass: "p4$$w0rd".to_owned(),
				},
				context: context.clone(),
			});

			assert_eq!(
				&result,
				&Err(LoginError::UserError(UserActionError::Authenticated(
					UserActionError::input(ErrorContext {
						action_type: UserActionType::Login,
						context: context.clone(),
					})
				)))
			);
		});
	}

	#[test]
	fn test_ok() {
		run_test(|_| {
			let context = user_context(UserTestOptions { user_id: None });

			let result = LoginAction::run(RequestInput {
				data: LoginData {
					name: "User 02".to_owned(),
					pass: "p4$$w0rd2".to_owned(),
				},
				context: context.clone(),
			});

			assert!(result.as_ref().is_ok());
			assert_eq!(
				result,
				Ok(LoginResult {
					id: 1,
					name: "User 02".to_string(),
				}),
			);
		});
	}
}
