#[cfg(test)]
pub mod tests {
	use mockito::{mock, Mock};
	use serde::de::DeserializeOwned;

	use crate::{
		core::external::{
			data::external_exception::ExternalException,
			definition::external::{
				tests::{ExternalMocker, ExternalTest, MockExternalMethod},
				ExternalAction,
			},
		},
		lib::data::str::Str,
	};

	fn test_external<I, O>(action: Str, method: MockExternalMethod, input: Option<I>) -> Option<O>
	where
		I: serde::Serialize,
		O: DeserializeOwned,
	{
		let method = match method {
			MockExternalMethod::Select => reqwest::Method::GET,
			MockExternalMethod::Insert => reqwest::Method::PUT,
			MockExternalMethod::Update => reqwest::Method::POST,
			MockExternalMethod::Delete => reqwest::Method::DELETE,
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

	fn mock_external<I, O>(
		action: Str,
		method: MockExternalMethod,
		input: Option<I>,
		output: Option<O>,
	) -> Mock
	where
		I: serde::Serialize,
		O: serde::Serialize,
	{
		let method = match method {
			MockExternalMethod::Select => "GET",
			MockExternalMethod::Insert => "PUT",
			MockExternalMethod::Update => "POST",
			MockExternalMethod::Delete => "DELETE",
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

	impl<I, O, T> ExternalAction<I, O> for T
	where
		I: serde::Serialize,
		O: DeserializeOwned,
		T: ExternalTest<I, O>,
	{
		fn run(input: I) -> Result<O, ExternalException> {
			Ok(test_external(Self::name(), Self::method(), Some(input)).unwrap())
		}
	}

	impl<I, O, T> ExternalMocker<I, O> for T
	where
		I: serde::Serialize,
		O: serde::Serialize + DeserializeOwned,
		T: ExternalTest<I, O>,
	{
		fn mock(input: I, output: O) -> mockito::Mock {
			mock_external(Self::name(), Self::method(), Some(input), Some(output))
		}
	}
}
