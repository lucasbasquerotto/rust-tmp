#[macro_use]
extern crate log;

mod business;
mod lib;

use std::fmt::Debug;

use business::action::action_data::general_action_data::{
	ActionRequestResult, Application, Request,
};

use business::action::action_data::moderator_action_data::{
	ModeratorActionType, ModeratorRequestContext, ModeratorSession,
};
use business::action::action_data::user_action_data::{
	UserActionType, UserRequestContext, UserSession,
};
use business::action::definition::moderator_action::ModeratorAction;
use business::action::definition::user_action::UserAction;
use business::action::implementation::login_action::LoginResult;
use lib::core::action::RequestInput;

use crate::business::action::action_data::general_action_data::ErrorData;
use crate::business::action::implementation::echo_action::{
	EchoErrorAction, EchoInfoAction, EchoWarnAction,
};
use crate::business::action::implementation::login_action::{LoginAction, LoginData};
use crate::business::action::implementation::logout_action::LogoutAction;
use crate::lib::core::action::ActionRequest;

use log::{Level, LevelFilter, Metadata, Record};

static MY_LOGGER: MyLogger = MyLogger;

pub fn main() {
	log::set_logger(&MY_LOGGER).unwrap();
	log::set_max_level(LevelFilter::Info);

	run("login".to_owned(), || login());
	run("logout".to_owned(), || logout());
	run("echo".to_owned(), || echo());
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

trait TestRequest<I: Debug, O: Debug, A> {
	fn test_request(data: I) -> ActionRequestResult<O>;
}

impl<I: Debug, O: Debug, A: UserAction<I, O>> TestRequest<I, O, UserActionType> for A {
	fn test_request(data: I) -> ActionRequestResult<O> {
		let context = UserRequestContext {
			application: Application {
				request_timeout: 1000,
			},
			session: UserSession { user_id: Some(123) },
			request: Request {
				ip: "1.2.3.4".to_string(),
			},
			action_type: Self::action_type(),
		};
		let input = Ok(RequestInput { context, data });
		Self::request(input)
	}
}

impl<I: Debug, O: Debug, A: ModeratorAction<I, O>> TestRequest<I, O, ModeratorActionType> for A {
	fn test_request(data: I) -> ActionRequestResult<O> {
		let context = ModeratorRequestContext {
			application: Application {
				request_timeout: 1000,
			},
			session: ModeratorSession {
				user_id: 123,
				allowed_actions: vec![],
			},
			request: Request {
				ip: "5.6.7.8".to_string(),
			},
			action_type: Self::action_type(),
		};
		let input = Ok(RequestInput { context, data });
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

	assert!(result.as_ref().is_err());
	assert_eq!(
		result.as_ref().unwrap_err(),
		&Some(ErrorData {
			key: "UserActionContextError::AUTHENTICATED".to_string(),
			msg: "You can't execute this action while authenticated.",
			params: None,
			meta: None
		}),
	);
	// assert!(result.as_ref().is_ok());
	// assert_eq!(
	// 	result.as_ref().unwrap(),
	// 	&LoginResult {
	// 		id: 1,
	// 		name: "User 01".to_string(),
	// 	},
	// );

	result
}

fn logout() -> ActionRequestResult<()> {
	let result = LogoutAction::test_request(());

	assert!(result.as_ref().is_ok());
	assert_eq!(result.as_ref().unwrap(), &());

	result
}

fn echo() -> ActionRequestResult<()> {
	let result = EchoInfoAction::test_request(());
	assert!(result.as_ref().is_ok());
	assert_eq!(result.as_ref().unwrap(), &());

	let result = EchoWarnAction::test_request(());
	assert!(result.as_ref().is_ok());
	assert_eq!(result.as_ref().unwrap(), &());

	let result = EchoErrorAction::test_request(());
	assert!(result.as_ref().is_ok());
	assert_eq!(result.as_ref().unwrap(), &());

	Ok(())
}
