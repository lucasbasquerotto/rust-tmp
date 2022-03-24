use crate::business::{
	action_type::user_action_type::UserActionType,
	data::{
		action_data::{ErrorContext, ErrorData, RequestContext, RequestInput},
		user_action_data::{UserActionError, UserNoAuthRequestContext, UserRequestContext},
	},
	definition::{
		action::{ActionError, ActionInput, ActionOutput, UserAction},
		action_helpers::ActionErrorHelper,
	},
};

#[derive(Debug, PartialEq)]
pub struct LoginData {
	pub name: String,
	pub pass: String,
}

impl ActionInput for LoginData {}

#[derive(Debug, PartialEq)]
pub struct LoginResult {
	pub id: u64,
	pub name: String,
}

impl ActionOutput for LoginResult {}

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

	fn description(&self) -> String {
		self.default_description()
	}
}

#[derive(Debug)]
pub struct LoginAction<T: RequestContext>(RequestInput<LoginData, T>);

impl UserAction<LoginData, LoginResult, LoginError> for LoginAction<UserNoAuthRequestContext> {
	fn action_type() -> UserActionType {
		UserActionType::Login
	}

	fn new_inner(
		input: Result<RequestInput<LoginData, UserRequestContext>, UserActionError>,
	) -> Result<Self, LoginError> {
		match input {
			Err(err) => Err(LoginError::UserError(err)),
			Ok(ok_input) => {
				let real_input = ok_input.to_no_auth(Self::action_type());

				match real_input {
					Err(err) => Err(LoginError::UserError(err)),
					Ok(real_ok_input) => Ok(Self(real_ok_input)),
				}
			}
		}
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

#[cfg(test)]
mod tests {
	use super::{LoginAction, LoginData, LoginError, LoginResult};
	use crate::business::action_type::user_action_type::UserActionType;
	use crate::business::data::action_data::ErrorContext;
	use crate::business::data::user_action_data::{UserActionError, UserErrorInput};
	use crate::tests::test_utils::tests::{run_test, user_context, TestRequest, UserOptions};

	#[test]
	fn test_1() {
		run_test(|_| {
			let context = user_context(UserOptions { user_id: Some(1) });

			let result = LoginAction::test_request(
				LoginData {
					name: "User 01".to_owned(),
					pass: "p4$$w0rd".to_owned(),
				},
				context.clone(),
			);

			assert_eq!(
				&result,
				&Err(LoginError::UserError(UserActionError::Authenticated(
					UserErrorInput {
						error_context: ErrorContext {
							action_type: UserActionType::Login,
							context: context.clone(),
						},
						data: (),
					}
				)))
			);
		});
	}

	#[test]
	fn test_2() {
		run_test(|_| {
			let context = user_context(UserOptions { user_id: None });

			let result = LoginAction::test_request(
				LoginData {
					name: "User 02".to_owned(),
					pass: "p4$$w0rd2".to_owned(),
				},
				context.clone(),
			);

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
