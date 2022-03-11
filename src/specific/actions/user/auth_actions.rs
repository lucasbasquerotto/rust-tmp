use crate::action::{Action, ActionCreator, ActionInput, ActionResult};

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

impl Action<LoginResult> for LoginAction {
	fn run(self) -> ActionResult<LoginResult> {
		let LoginData { name, pass } = &self.0.request;
		println!("login: {name} ({pass})");
		let result = LoginResult {
			id: 1,
			name: name.to_string(),
		};
		Ok(result)
	}
}

impl ActionCreator<LoginData, LoginResult, Self> for LoginAction {
	fn new(input: ActionInput<LoginData>) -> Self {
		Self(input)
	}
}

/////////////////////////////////////////////////////////////////////////////////////
// LOGOUT
/////////////////////////////////////////////////////////////////////////////////////

pub struct LogoutAction(ActionInput<()>);

impl Action<()> for LogoutAction {
	fn run(self) -> ActionResult<()> {
		println!("logout");
		Ok(())
	}
}

impl ActionCreator<(), (), Self> for LogoutAction {
	fn new(input: ActionInput<()>) -> Self {
		Self(input)
	}
}
