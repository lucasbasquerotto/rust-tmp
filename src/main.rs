#[macro_use]
extern crate log;

mod business;
mod lib;

use std::fmt::Debug;

use business::action::action_data::{ActionRequestResult, Application, Request, Session};

use business::action::action_type::user_action_type::UserRequestInfo;
use business::action::definition::user_action::UserAction;
use business::action::implementation::login_action::LoginResult;
use lib::core::action_core::RequestInput;

use crate::business::action::implementation::login_action::{LoginAction, LoginData};
use crate::business::action::implementation::logout_action::LogoutAction;
use crate::lib::base::action::ActionRequest;

use log::{Level, LevelFilter, Metadata, Record};

static MY_LOGGER: MyLogger = MyLogger;

pub fn main() {
	log::set_logger(&MY_LOGGER).unwrap();
	log::set_max_level(LevelFilter::Info);

	run("login".to_owned(), || login());
	run("logout".to_owned(), || logout());
}

struct MyLogger;

impl log::Log for MyLogger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() <= Level::Info
	}

	fn log(&self, record: &Record) {
		if self.enabled(record.metadata()) {
			println!("{} - {}", record.level(), record.args());
		}
	}
	fn flush(&self) {}
}

trait TestRequest<I: Debug, O: Debug> {
	fn test_request(data: I) -> ActionRequestResult<O>;
}

impl<I: Debug, O: Debug, A: UserAction<I, O>> TestRequest<I, O> for A {
	fn test_request(data: I) -> ActionRequestResult<O> {
		let info = UserRequestInfo {
			application: Application {
				request_timeout: 1000,
			},
			session: Session { user_id: Some(123) },
			request: Request {
				ip: "1.2.3.4".to_string(),
			},
			action_type: Self::action_type(),
		};
		let input = Ok(RequestInput { info, data });
		Self::request(input)
	}
}

fn run<T: Debug, F: Fn() -> T>(name: String, function: F) {
	println!("{name} started...");
	let result = function();
	println!("{name} ended -> result: {:?}\n", result);
}

fn login() -> ActionRequestResult<LoginResult> {
	let result = LoginAction::test_request(LoginData {
		name: "User 01".to_owned(),
		pass: "p4$$w0rd".to_owned(),
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

fn logout() -> ActionRequestResult<()> {
	let result = LogoutAction::test_request(());

	assert!(result.as_ref().is_ok());
	assert_eq!(result.as_ref().unwrap(), &());

	result
}
