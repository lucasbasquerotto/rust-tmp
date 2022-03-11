use crate::action::{Action, ActionInput, ActionResult};
use crate::action::{Cast, GeneralActionMainResult, GeneralActionOutput, GeneralActionResult};

pub struct LoginData {
	pub name: String,
	pub pass: String,
}

#[derive(Debug)]
pub struct LoginResult {
	pub id: u64,
	pub name: String,
}

impl GeneralActionOutput for LoginResult {}

pub enum UserAction {
	LOGIN(ActionInput<LoginData>),
}

impl Action<GeneralActionMainResult> for UserAction {
	fn run(self) -> GeneralActionResult {
		match self {
			UserAction::LOGIN(data) => login(&data).cast(),
		}
	}
}

pub fn login(data: &ActionInput<LoginData>) -> ActionResult<LoginResult> {
	let LoginData { name, pass } = &data.request;
	println!("login: {name} ({pass})");
	let result = LoginResult {
		id: 1,
		name: name.to_string(),
	};
	Ok(result)
}