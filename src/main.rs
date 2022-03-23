#[macro_use]
extern crate log;

mod business;

use std::fmt::Debug;

use business::action_type::action_type::ActionType;
use business::action_type::moderator_action_type::ModeratorActionType;
use business::action_type::user_action_type::UserActionType;
use business::data::action_data::{Application, Request, RequestInput};

use business::data::moderator_action_data::{ModeratorRequestContext, ModeratorSession};
use business::data::user_action_data::{UserRequestContext, UserSession};
use business::definition::action::{ActionError, UserAction};
use business::definition::action::{ActionInput, ActionOutput, ModeratorAction};
use business::definition::action_helpers::DescriptiveRequestContext;
use business::main::login_action::LoginResult;

use crate::business::data::action_data::ErrorData;
use crate::business::main::echo::echo_error_action::EchoErrorAction;
use crate::business::main::echo::echo_info_action::EchoInfoAction;
use crate::business::main::echo::echo_warn_action::EchoWarnAction;
use crate::business::main::login_action::{LoginAction, LoginData};
use crate::business::main::logout_action::LogoutAction;

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

trait TestRequestOptions {}

#[derive(Debug, Clone)]
struct UserOptions {
	pub user_id: Option<u64>,
}

impl TestRequestOptions for UserOptions {}

#[derive(Debug, Clone)]
struct ModeratorOptions {
	pub allowed_actions: Vec<u32>,
}

impl TestRequestOptions for ModeratorOptions {}

trait TestRequest<
	T: ActionType,
	C: DescriptiveRequestContext,
	I: ActionInput,
	O: ActionOutput,
	E: ActionError<T, C>,
	P: TestRequestOptions,
	A: ActionType,
>
{
	fn test_request(data: I, options: P) -> Result<O, E>;
}

impl<I, O, E, A>
	TestRequest<UserActionType, UserRequestContext, I, O, E, UserOptions, UserActionType> for A
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError<UserActionType, UserRequestContext>,
	A: UserAction<I, O, E>,
{
	fn test_request(data: I, options: UserOptions) -> Result<O, E> {
		let context = UserRequestContext {
			application: Application {
				request_timeout: 1000,
			},
			session: UserSession {
				user_id: options.user_id,
			},
			request: Request {
				ip: "1.2.3.4".to_string(),
			},
		};
		let input = RequestInput { context, data };
		Self::run(input)
	}
}

impl<I, O, E, A>
	TestRequest<
		ModeratorActionType,
		ModeratorRequestContext,
		I,
		O,
		E,
		ModeratorOptions,
		ModeratorActionType,
	> for A
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError<ModeratorActionType, ModeratorRequestContext>,
	A: ModeratorAction<I, O, E>,
{
	fn test_request(data: I, options: ModeratorOptions) -> Result<O, E> {
		let context = ModeratorRequestContext {
			application: Application {
				request_timeout: 1000,
			},
			session: ModeratorSession {
				user_id: 123,
				allowed_actions: options.allowed_actions,
			},
			request: Request {
				ip: "5.6.7.8".to_string(),
			},
		};
		let input = RequestInput { context, data };
		Self::run(input)
	}
}

fn run<T: Debug, F: Fn() -> T>(name: String, function: F) {
	println!("=========================================");
	println!("{name} started...");
	println!("-----------------------------------------");
	function();
	println!("-----------------------------------------");
	println!("{name} ended");
	println!("=========================================");
	println!();
}

fn login() {
	let result = LoginAction::test_request(
		LoginData {
			name: "User 01".to_owned(),
			pass: "p4$$w0rd".to_owned(),
		},
		UserOptions { user_id: Some(1) },
	);

	println!("result: {:?}", result);

	assert!(result.as_ref().is_err());
	assert_eq!(
		result.as_ref().unwrap_err().public_error(),
		Some(ErrorData {
			msg: "You can't execute this action while authenticated.".to_string(),
			params: None,
		}),
	);

	println!("-----------------------------------------");

	let result = LoginAction::test_request(
		LoginData {
			name: "User 01".to_owned(),
			pass: "p4$$w0rd".to_owned(),
		},
		UserOptions { user_id: None },
	);

	println!("result: {:?}", result);

	assert!(result.as_ref().is_ok());
	assert_eq!(
		result.as_ref().unwrap(),
		&LoginResult {
			id: 1,
			name: "User 01".to_string(),
		},
	);
}

fn logout() {
	let result = LogoutAction::test_request((), UserOptions { user_id: None });

	println!("result: {:?}", result);

	assert!(result.as_ref().is_ok());
	assert_eq!(result.as_ref().unwrap(), &());
}

fn echo() {
	let options = ModeratorOptions {
		allowed_actions: vec![
			EchoInfoAction::action_type().id(),
			EchoWarnAction::action_type().id(),
			EchoErrorAction::action_type().id(),
		],
	};

	let result = EchoInfoAction::test_request((), options.clone());
	println!("result: {:?}", result);
	assert!(result.as_ref().is_ok());
	assert_eq!(result.as_ref().unwrap(), &());

	println!("-----------------------------------------");

	let result = EchoWarnAction::test_request((), options.clone());
	println!("result: {:?}", result);
	assert!(result.as_ref().is_ok());
	assert_eq!(result.as_ref().unwrap(), &());

	println!("-----------------------------------------");

	let result = EchoErrorAction::test_request((), options.clone());
	println!("result: {:?}", result);
	assert!(result.as_ref().is_ok());
	assert_eq!(result.as_ref().unwrap(), &());
}
