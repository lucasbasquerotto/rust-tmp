use crate::core::external::data::external_exception::ExternalException;

#[rocket::async_trait]
pub trait ExternalAction<I, O> {
	async fn run(input: I) -> Result<O, ExternalException>;
}

#[cfg(test)]
pub mod tests {
	use super::ExternalAction;
	use crate::lib::data::str::Str;

	#[allow(dead_code)]
	pub enum MockExternalMethod {
		Select,
		Insert,
		Update,
		Delete,
	}

	pub trait ExternalTest<I, O>: ExternalAction<I, O> {
		fn name() -> Str;
		fn method() -> MockExternalMethod;
	}

	pub trait ExternalMocker<I, O>: ExternalTest<I, O> {
		fn mock(input: I, output: O) -> mockito::Mock;
	}
}
