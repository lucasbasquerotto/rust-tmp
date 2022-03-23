#[cfg(test)]
pub mod tests {
	extern crate lazy_static;
	extern crate log;

	use std::fmt::Debug;
	use std::sync::{Arc, Mutex};

	use business::action_type::action_type::ActionType;
	use business::action_type::moderator_action_type::ModeratorActionType;
	use business::action_type::user_action_type::UserActionType;
	use business::data::action_data::{Application, Request, RequestInput};

	use business::data::moderator_action_data::{ModeratorRequestContext, ModeratorSession};
	use business::data::user_action_data::{UserRequestContext, UserSession};
	use business::definition::action::{ActionError, UserAction};
	use business::definition::action::{ActionInput, ActionOutput, ModeratorAction};
	use business::definition::action_helpers::DescriptiveRequestContext;

	use log::{Level, Metadata, Record};

	lazy_static::lazy_static! {
		static ref MY_LOGGER: MyLogger = MyLogger(Arc::new(Mutex::new(vec![])));
	}

	struct MyLogger(Arc<Mutex<Vec<String>>>);

	impl log::Log for MyLogger {
		fn enabled(&self, metadata: &Metadata) -> bool {
			metadata.level() <= Level::Info
		}

		fn log(&self, record: &Record) {
			if self.enabled(record.metadata()) {
				println!("log: {}", record.args());
				self.0.lock().unwrap().insert(
					0,
					format!(
						"{level} - {args}",
						level = record.level(),
						args = record.args()
					),
				);
			}
		}
		fn flush(&self) {}
	}

	pub fn pop_log() -> Option<String> {
		MY_LOGGER.0.lock().unwrap().pop()
	}

	pub trait TestRequestOptions {}

	#[derive(Debug, Clone)]
	pub struct UserOptions {
		pub user_id: Option<u64>,
	}

	impl TestRequestOptions for UserOptions {}

	#[derive(Debug, Clone)]
	pub struct ModeratorOptions {
		pub allowed_actions: Vec<u32>,
	}

	impl TestRequestOptions for ModeratorOptions {}

	pub trait TestRequest<
		T: ActionType,
		C: DescriptiveRequestContext,
		I: ActionInput,
		O: ActionOutput,
		E: ActionError<T, C>,
		A: ActionType,
	>
	{
		fn test_request(data: I, context: C) -> Result<O, E>;
	}

	pub fn user_context(options: UserOptions) -> UserRequestContext {
		UserRequestContext {
			application: Application {
				request_timeout: 1000,
			},
			session: UserSession {
				user_id: options.user_id,
			},
			request: Request {
				ip: "1.2.3.4".to_string(),
			},
		}
	}

	impl<I, O, E, A> TestRequest<UserActionType, UserRequestContext, I, O, E, UserActionType> for A
	where
		I: ActionInput,
		O: ActionOutput,
		E: ActionError<UserActionType, UserRequestContext>,
		A: UserAction<I, O, E>,
	{
		fn test_request(data: I, context: UserRequestContext) -> Result<O, E> {
			let input = RequestInput { context, data };
			Self::run(input)
		}
	}

	pub fn moderator_context(options: ModeratorOptions) -> ModeratorRequestContext {
		ModeratorRequestContext {
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
		}
	}

	impl<I, O, E, A>
		TestRequest<ModeratorActionType, ModeratorRequestContext, I, O, E, ModeratorActionType> for A
	where
		I: ActionInput,
		O: ActionOutput,
		E: ActionError<ModeratorActionType, ModeratorRequestContext>,
		A: ModeratorAction<I, O, E>,
	{
		fn test_request(data: I, context: ModeratorRequestContext) -> Result<O, E> {
			let input = RequestInput { context, data };
			Self::run(input)
		}
	}
}
