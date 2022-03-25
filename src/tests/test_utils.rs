#[cfg(test)]
pub mod tests {

	use std::fmt::Debug;
	use std::sync::{Arc, Mutex};

	use business::action_type::moderator_action_type::ModeratorActionType;
	use business::data::action_data::{Application, Request};

	use business::data::moderator_action_data::{ModeratorRequestContext, ModeratorSession};
	use business::data::user_action_data::{UserRequestContext, UserSession};

	use log::{Level, LevelFilter, Metadata, Record};

	use crate::business::data::automatic_action_data::{AutomaticRequest, AutomaticRequestContext};

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

	pub trait TestRequestOptions {}

	#[derive(Debug, Clone)]
	pub struct UserOptions {
		pub user_id: Option<u64>,
	}

	impl TestRequestOptions for UserOptions {}

	#[derive(Debug, Clone)]
	pub struct ModeratorOptions {
		pub admin: bool,
		pub allowed_actions: Vec<ModeratorActionType>,
	}

	impl TestRequestOptions for ModeratorOptions {}

	#[derive(Debug, Clone)]
	pub struct AutomaticOptions {
		pub internal: bool,
	}

	impl TestRequestOptions for AutomaticOptions {}

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

	pub fn moderator_context(options: ModeratorOptions) -> ModeratorRequestContext {
		ModeratorRequestContext {
			application: Application {
				request_timeout: 1000,
			},
			session: ModeratorSession {
				user_id: 123,
				admin: options.admin,
				allowed_actions: options.allowed_actions,
			},
			request: Request {
				ip: "5.6.7.8".to_string(),
			},
		}
	}

	pub fn automatic_context(options: AutomaticOptions) -> AutomaticRequestContext {
		AutomaticRequestContext {
			application: Application {
				request_timeout: 1000,
			},
			request: if options.internal {
				AutomaticRequest::Internal
			} else {
				AutomaticRequest::Hook(Request {
					ip: "0.1.2.3".to_string(),
				})
			},
		}
	}

	pub struct TestHelper;

	impl TestHelper {
		pub fn pop_log(&self) -> Option<String> {
			MY_LOGGER.0.lock().unwrap().pop()
		}

		fn clear_log(&self) {
			MY_LOGGER.0.lock().unwrap().clear();
		}
	}

	pub fn init() {
		log::set_logger(&*MY_LOGGER).unwrap();
		log::set_max_level(LevelFilter::Info);
	}

	pub fn run_test<F: Fn(&TestHelper)>(function: F) {
		let helper = TestHelper;
		helper.clear_log();
		function(&helper);
		assert_eq!(helper.pop_log(), None, "Verify that no log remained");
	}
}
