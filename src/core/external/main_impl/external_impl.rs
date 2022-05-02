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

	pub async fn test_external<I, O>(action: Str, method: MockExternalMethod, input: I) -> O
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

		reqwest::Client::new()
			.request(method, url)
			.body(serde_json::to_string(&input).unwrap())
			.send()
			.await
			.unwrap()
			.error_for_status()
			.unwrap()
			.json::<O>()
			.await
			.unwrap()
	}

	pub fn mock_external<I, O>(action: Str, method: MockExternalMethod, input: I, output: O) -> Mock
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
			.match_body(serde_json::to_string(&input).unwrap().as_ref())
			.with_body(output.as_ref())
			.with_status(200)
			.create()
	}

	#[rocket::async_trait]
	impl<I: 'static, O, T> ExternalAction<I, O> for T
	where
		I: serde::Serialize + Send + Sync,
		O: DeserializeOwned,
		T: ExternalTest<I, O> + Send + 'static,
	{
		async fn run(input: I) -> Result<O, ExternalException> {
			Ok(test_external(Self::name(), Self::method(), input).await)
		}
	}

	impl<I, O, T> ExternalMocker<I, O> for T
	where
		I: serde::Serialize,
		O: serde::Serialize + DeserializeOwned,
		T: ExternalTest<I, O>,
	{
		fn mock(input: I, output: O) -> mockito::Mock {
			mock_external(Self::name(), Self::method(), input, output)
		}
	}

	#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
	struct TestInput {
		nickname: Str,
	}

	#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
	struct TestOutput {
		id: u64,
		name: Str,
	}

	#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
	enum TestEnumInput {
		First,
		Last,
	}

	#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
	enum TestEnumOutput {
		First,
		Last,
	}

	#[tokio::test]
	async fn test_external_mock_call() {
		//run_test(|_| async {});
		let _m1 = mock_external("test".into(), MockExternalMethod::Insert, (), ());

		let input1 = TestInput {
			nickname: "test-01".into(),
		};

		let _m2 = mock_external(
			"test-out".into(),
			MockExternalMethod::Insert,
			input1.clone(),
			(),
		);

		let output1 = TestOutput {
			id: 1,
			name: "Test 01".into(),
		};

		let _m3 = mock_external(
			"test-out".into(),
			MockExternalMethod::Select,
			(),
			output1.clone(),
		);

		let input2 = TestInput {
			nickname: "test-01".into(),
		};

		let output2 = TestOutput {
			id: 2,
			name: "Test 02".into(),
		};

		let _m4 = mock_external(
			"test-out".into(),
			MockExternalMethod::Select,
			input2.clone(),
			output2.clone(),
		);

		let _m5 = mock_external(
			"test-enum".into(),
			MockExternalMethod::Update,
			TestEnumInput::First,
			TestEnumOutput::First,
		);

		let _m6 = mock_external(
			"test-enum".into(),
			MockExternalMethod::Update,
			TestEnumInput::Last,
			TestEnumOutput::Last,
		);

		let result: () = test_external("test".into(), MockExternalMethod::Insert, ()).await;
		assert_eq!(&result, &(), "no input / no output");

		let result: () = test_external("test-out".into(), MockExternalMethod::Insert, input1).await;
		assert_eq!(&result, &(), "with input / no output");

		let result: TestOutput =
			test_external("test-out".into(), MockExternalMethod::Select, ()).await;
		assert_eq!(&result, &output1, "no input / with output");

		let result: TestOutput =
			test_external("test-out".into(), MockExternalMethod::Select, input2).await;
		assert_eq!(&result, &output2, "with input / with output");

		let result: TestEnumOutput = test_external(
			"test-enum".into(),
			MockExternalMethod::Update,
			TestEnumInput::First,
		)
		.await;
		assert_eq!(&result, &TestEnumOutput::First, "first enum");

		let result: TestEnumOutput = test_external(
			"test-enum".into(),
			MockExternalMethod::Update,
			TestEnumInput::Last,
		)
		.await;
		assert_eq!(&result, &TestEnumOutput::Last, "last enum");
	}
}
