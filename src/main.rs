mod business;
mod lib;
mod test;

use std::fmt::Debug;

use crate::business::actions::user::auth::login_action::{LoginAction, LoginData, LoginResult};
use crate::business::actions::user::auth::logout_action::LogoutAction;
use crate::lib::base::action::ActionRequest;
use crate::lib::core::action_core::{ActionInput, ActionRequestResult, ActionResult};

pub fn main() {
	run("login".to_owned(), || login());
	run("logout".to_owned(), || logout());
}

fn input<T: Debug>(request: T) -> ActionResult<ActionInput<T>> {
	Ok(ActionInput { request })
}

fn run<T: Debug, F: Fn() -> T>(name: String, function: F) {
	println!("{name} started...");
	let result = function();
	println!("{name} ended -> result: {:?}\n", result);
}

fn login() -> ActionRequestResult<LoginResult> {
	let result = LoginAction::request(input(LoginData {
		name: "User 01".to_owned(),
		pass: "p4$$w0rd".to_owned(),
	}));

	assert!(result.as_ref().is_ok());
	assert_eq!(
		result.as_ref().unwrap(),
		&LoginResult {
			id: 1,
			name: "User 01".to_string(),
		},
	);

	result
}

fn logout() -> ActionRequestResult<()> {
	let result = LogoutAction::request(input(()));

	assert!(result.as_ref().is_ok());
	assert_eq!(result.as_ref().unwrap(), &());

	result
}
