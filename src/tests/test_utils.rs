#[cfg(test)]
pub mod tests {
	use log::{Level, LevelFilter, Metadata, Record};
	use std::sync::{Arc, Mutex};

	use crate::lib::data::str::Str;

	lazy_static::lazy_static! {
		static ref MY_LOGGER: MyLogger = MyLogger(Arc::new(Mutex::new(vec![])));
	}

	struct MyLogger(Arc<Mutex<Vec<Str>>>);

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
					)
					.into(),
				);
			}
		}
		fn flush(&self) {}
	}

	pub struct TestHelper;

	impl TestHelper {
		pub fn pop_log(&self) -> Option<Str> {
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
