use crate::action::{Action, ActionCreator, ActionInput, ActionResult, GeneralActionCreator};

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

pub struct LoginAction<'a>(&'a ActionInput<LoginData>);

impl<'a> Action<LoginResult> for LoginAction<'a> {
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

pub struct Login();

impl<'a> ActionCreator<'a, LoginData, LoginResult, LoginAction<'a>> for Login {
	fn new(&'a self, input: &'a ActionInput<LoginData>) -> LoginAction {
		LoginAction(input)
	}
}

impl GeneralActionCreator for Login {}

/////////////////////////////////////////////////////////////////////////////////////

// pub struct LoginAction<'a>(&'a ActionInput<LoginData>);

// pub struct Logout();
