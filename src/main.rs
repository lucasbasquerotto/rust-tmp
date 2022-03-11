mod action;
mod specific;

use std::fmt::Debug;

use crate::action::{ActionInput, ActionRequest, ErrorData};
use crate::specific::actions::user::auth_actions::{
	LoginAction, LoginData, LoginResult, LogoutAction,
};

pub fn main() {
	run("login".to_owned(), || login());
	run("logout".to_owned(), || logout());
}

fn input<T: Debug>(request: T) -> ActionInput<T> {
	ActionInput { request }
}

fn run<T: Debug, F: Fn() -> T>(name: String, function: F) {
	println!("{name} started...");
	let result = function();
	println!("{name} ended -> result: {:?}\n", result);
}

fn login() -> Result<LoginResult, ErrorData> {
	let result = LoginAction::run(|| {
		input(LoginData {
			name: "User 01".to_owned(),
			pass: "p4$$w0rd".to_owned(),
		})
	});

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

fn logout() -> Result<(), ErrorData> {
	let result = LogoutAction::run(|| input(()));

	assert!(result.as_ref().is_ok());
	assert_eq!(result.as_ref().unwrap(), &());

	result
}
