use crate::{
	business::action::{
		action_type::user_action_type::UserActionType,
		data::user_action_data::UserRequestContext,
		definition::business_action::{UserAction, UserActionResult},
	},
	lib::core::action::{RequestContext, RequestInput},
};

#[derive(Debug)]
pub struct LoginData {
	pub name: String,
	pub pass: String,
}

#[derive(Debug, PartialEq)]
pub struct LoginResult {
	pub id: u64,
	pub name: String,
}

#[derive(Debug)]
pub struct LoginAction<T: RequestContext>(RequestInput<LoginData, T>);

impl UserAction<LoginData, LoginResult> for LoginAction<UserRequestContext> {
	fn action_type() -> UserActionType {
		UserActionType::Login
	}

	fn new(input: RequestInput<LoginData, UserRequestContext>) -> Self {
		Self(input)
	}

	fn input(&self) -> &RequestInput<LoginData, UserRequestContext> {
		&self.0
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
