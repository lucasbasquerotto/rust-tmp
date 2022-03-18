use crate::{
	business::action::{
		action_type::user_action_type::UserActionType,
		data::user_action_data::{UserNoAuthRequestContext, UserRequestContext},
		definition::business_action::{ActionInput, ActionOutput, UserAction, UserActionResult},
	},
	lib::core::action::{RequestContext, RequestInput},
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

#[derive(Debug)]
pub struct LoginAction<T: RequestContext>(RequestInput<LoginData, T>);

impl UserAction<LoginData, LoginResult> for LoginAction<UserNoAuthRequestContext> {
	fn action_type() -> UserActionType {
		UserActionType::Login
	}

	fn new(input: RequestInput<LoginData, UserRequestContext>) -> UserActionResult<Self> {
		let real_input = input.to_no_auth()?;
		Ok(Self(real_input))
	}

	fn run_inner(self) -> UserActionResult<LoginResult> {
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
