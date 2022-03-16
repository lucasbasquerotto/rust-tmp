use crate::{
	business::action::{
		action_type::user_action_type::{UserActionType, UserRequestInfo},
		definition::user_action::{UserAction, UserActionResult},
	},
	lib::core::action_core::{RequestInfo, RequestInput},
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
pub struct LoginAction<T: RequestInfo>(RequestInput<LoginData, T>);

impl UserAction<LoginData, LoginResult> for LoginAction<UserRequestInfo> {
	fn action_type() -> UserActionType {
		UserActionType::LOGIN
	}

	fn new(input: RequestInput<LoginData, UserRequestInfo>) -> Self {
		Self(input)
	}

	fn input(self) -> RequestInput<LoginData, UserRequestInfo> {
		self.0
	}

	fn run(self) -> UserActionResult<LoginResult> {
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
