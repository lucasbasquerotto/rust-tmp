use crate::{
	business::actions::{
		business_action::ActionResult,
		contexts::user_action::{UserAction, UserActionType, UserRequestInfo},
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

// pub struct LoginAction(ActionInput<LoginData>);

// impl CoreAction<LoginData, LoginResult> for LoginAction {
// 	fn get_type() -> CoreActionType {
// 		Box::new(UserAction::LOGIN)
// 	}

// 	fn new(input: ActionInput<LoginData>) -> Self {
// 		Self(input)
// 	}

// 	fn run(self) -> ActionResult<LoginResult> {
// 	}
// }

#[derive(Debug)]
pub struct LoginAction<T: RequestInfo>(RequestInput<LoginData, T>);

impl UserAction<LoginData, LoginResult> for LoginAction<UserRequestInfo> {
	fn action_type() -> UserActionType {
		UserActionType::LOGIN
	}

	fn new(input: RequestInput<LoginData, UserRequestInfo>) -> Self {
		Self(input)
	}

	fn run(self) -> ActionResult<LoginResult> {
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
