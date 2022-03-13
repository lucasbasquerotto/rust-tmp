use crate::{
	business::actions::user_actions::UserAction,
	lib::core::action_core::{ActionInput, ActionResult, CoreAction, CoreActionType},
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

pub struct LoginAction(ActionInput<LoginData>);

impl CoreAction<LoginData, LoginResult> for LoginAction {
	fn get_type() -> CoreActionType {
		Box::new(UserAction::LOGIN)
	}

	fn new(input: ActionInput<LoginData>) -> Self {
		Self(input)
	}

	fn run(self) -> ActionResult<LoginResult> {
		let LoginAction(input) = &self;
		let LoginData { name, pass } = &input.request;
		println!("login: {name} ({pass})");
		let result = LoginResult {
			id: 1,
			name: name.to_string(),
		};
		Ok(result)
	}
}
