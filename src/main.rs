#[macro_use]
extern crate log;

mod business;
mod lib;

use std::fmt::Debug;

use business::action::action_type::action_type::BusinessActionType;
use business::action::action_type::moderator_action_type::ModeratorActionType;
use business::action::action_type::user_action_type::UserActionType;
use business::action::data::action_data::{ActionRequestResult, Application, Request};

use business::action::data::moderator_action_data::{ModeratorRequestContext, ModeratorSession};
use business::action::data::user_action_data::{UserRequestContext, UserSession};
use business::action::definition::action_error::BusinessException;
use business::action::definition::business_action::{ActionError, UserAction};
use business::action::definition::business_action::{ActionInput, ActionOutput, ModeratorAction};
use business::action::main::login_action::{LoginError, LoginResult};
use business::action::main::logout_action::LogoutError;
use lib::core::action::{Action, RequestInput};

use crate::business::action::data::action_data::ErrorData;
use crate::business::action::main::echo::echo_error_action::EchoErrorAction;
use crate::business::action::main::echo::echo_info_action::EchoInfoAction;
use crate::business::action::main::echo::echo_warn_action::EchoWarnAction;
use crate::business::action::main::login_action::{LoginAction, LoginData};
use crate::business::action::main::logout_action::LogoutAction;

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

trait TestRequest<I: ActionInput, O: ActionOutput, E: ActionError, A> {
	fn test_request(data: I) -> Result<O, E>;
}

impl<I, O, E, A> TestRequest<I, O, E, UserActionType> for A
where
	I: ActionInput,
	O: ActionOutput,
	E: BusinessException<UserActionType, UserRequestContext> + ActionError,
	A: UserAction<I, O, E> + Action<UserRequestContext, I, O, E, UserActionType>,
{
	fn test_request(data: I) -> Result<O, E> {
		let context = UserRequestContext {
			application: Application {
				request_timeout: 1000,
			},
			session: UserSession { user_id: Some(123) },
			request: Request {
				ip: "1.2.3.4".to_string(),
			},
		};
		let input = RequestInput { context, data };
		let action = Self::new(input)?;
		action.run()
	}
}

impl<I, O, E, A> TestRequest<I, O, E, ModeratorActionType> for A
where
	I: ActionInput,
	O: ActionOutput,
	E: ActionError,
	A: ModeratorAction<I, O, E> + Action<ModeratorRequestContext, I, O, E, ModeratorActionType>,
{
	fn test_request(data: I) -> Result<O, E> {
		let context = ModeratorRequestContext {
			application: Application {
				request_timeout: 1000,
			},
			session: ModeratorSession {
				user_id: 123,
				allowed_actions: vec![
					EchoInfoAction::action_type().id(),
					EchoWarnAction::action_type().id(),
					EchoErrorAction::action_type().id(),
				],
			},
			request: Request {
				ip: "5.6.7.8".to_string(),
			},
		};
		let input = RequestInput { context, data };
		let action = Self::new(input)?;
		action.run()
	}
}

fn run<T: Debug, F: Fn() -> T>(name: String, function: F) {
	println!("{name} started...");
	let result = function();
	println!("{name} ended -> result: {:?}\n", result);
}

fn login() -> Result<LoginResult, LoginError> {
	let result = LoginAction::test_request(LoginData {
		name: "User 01".to_owned(),
		pass: "p4$$w0rd".to_owned(),
	});

	assert!(result.as_ref().is_err());
	assert_eq!(
		result.as_ref().unwrap_err().public_error(),
		Some(ErrorData {
			msg: "You can't execute this action while authenticated.".to_string(),
			params: None,
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

fn logout() -> Result<(), LogoutError> {
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
