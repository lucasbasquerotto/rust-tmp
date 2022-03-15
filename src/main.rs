mod business;
mod lib;

use std::fmt::Debug;

use business::action::business_action::{
	ActionRequestResult, ActionResult, Application, Request, Session,
};

use business::action::r#impl::login_action::LoginResult;
use business::action::r#type::user_action_type::UserRequestInfo;
use lib::core::action_core::RequestInput;

use crate::business::action::r#impl::login_action::{LoginAction, LoginData};
use crate::business::action::r#impl::logout_action::LogoutAction;
use crate::lib::base::action::ActionRequest;

pub fn main() {
	run("login".to_owned(), || login());
	run("logout".to_owned(), || logout());
}

fn input<T: Debug>(data: T) -> ActionResult<RequestInput<T, UserRequestInfo>> {
	let info = UserRequestInfo {
		application: Application {
			request_timeout: 1000,
		},
		session: Session { user_id: 123 },
		request: Request {
			ip: "1.2.3.4".to_string(),
		},
	};
	Ok(RequestInput { info, data })
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
