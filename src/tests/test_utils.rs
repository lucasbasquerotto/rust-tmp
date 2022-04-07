#[cfg(test)]
pub mod tests {
	use crate::lib::data::str::Str;
	use log::{Level, LevelFilter, Metadata, Record};
	use mockito::{mock, Mock};
	use serde::de::DeserializeOwned;
	use std::sync::{Arc, Mutex};

	lazy_static::lazy_static! {
		static ref MY_LOGGER: MyLogger = MyLogger(Arc::new(Mutex::new(vec![])));
	}

	pub enum MockDaoMethod {
		Select,
		Insert,
		Update,
		Delete,
	}

	pub fn test_dao<I, O>(action: Str, method: MockDaoMethod, input: Option<I>) -> Option<O>
	where
		I: serde::Serialize,
		O: DeserializeOwned,
	{
		let method = match method {
			MockDaoMethod::Select => reqwest::Method::GET,
			MockDaoMethod::Insert => reqwest::Method::PUT,
			MockDaoMethod::Update => reqwest::Method::POST,
			MockDaoMethod::Delete => reqwest::Method::DELETE,
		};

		let url = format!(
			"{host}/{subpath}/{action}",
			host = mockito::SERVER_URL,
			subpath = "mock/dao"
		);

		let client = reqwest::blocking::Client::new();
		let builder = client.request(method, url).body(match input {
			Some(input) => serde_json::to_string(&input).unwrap(),
			None => "".to_string(),
		});

		builder
			.send()
			.unwrap()
			.error_for_status()
			.unwrap()
			.json::<O>()
			.map(Some)
			.unwrap_or(None)
	}

	pub fn mock_dao<I, O>(
		action: Str,
		method: MockDaoMethod,
		input: Option<I>,
		output: Option<O>,
	) -> Mock
	where
		I: serde::Serialize,
		O: serde::Serialize,
	{
		let method = match method {
			MockDaoMethod::Select => "GET",
			MockDaoMethod::Insert => "PUT",
			MockDaoMethod::Update => "POST",
			MockDaoMethod::Delete => "DELETE",
		};
		let output = serde_json::to_string(&output).unwrap();
		mock(method, format!("/mock/dao/{action}").as_ref())
			.match_body(
				match input {
					Some(input) => serde_json::to_string(&input).unwrap(),
					None => "".to_string(),
				}
				.as_ref(),
			)
			.with_body(output.as_ref())
			.with_status(200)
			.create()
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
