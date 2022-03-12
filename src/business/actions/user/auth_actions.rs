use crate::{
	business::user_actions::UserAction,
	lib::core::action_core::{ActionInput, ActionResult, CoreAction, CoreActionType},
};

/////////////////////////////////////////////////////////////////////////////////////
// LOGIN
/////////////////////////////////////////////////////////////////////////////////////

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
	fn new(input: ActionInput<LoginData>) -> Self {
		Self(input)
	}

	fn get_type(&self) -> CoreActionType {
		Box::new(UserAction::LOGIN)
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

/////////////////////////////////////////////////////////////////////////////////////
// LOGOUT
/////////////////////////////////////////////////////////////////////////////////////

pub struct LogoutAction(ActionInput<()>);

impl CoreAction<(), ()> for LogoutAction {
	fn new(input: ActionInput<()>) -> Self {
		Self(input)
	}

	fn get_type(&self) -> CoreActionType {
		Box::new(UserAction::LOGOUT)
	}

	fn run(self) -> ActionResult<()> {
		println!("logout");
		Ok(())
	}
}
