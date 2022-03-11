use crate::{
	action::{ActionInput, ActionResult},
	specific::user_actions::{LoginData, LoginResult},
};

fn login(data: &ActionInput<LoginData>) -> ActionResult<LoginResult> {
	let LoginData { name, pass } = &data.request;
	println!("login: {name} ({pass})");
	let result = LoginResult {
		id: 1,
		name: name.to_string(),
	};
	Ok(result)
}

pub struct Login();
