use std::fmt::Debug;

pub enum ActionScope {
	USER,
	// MODERATOR,
	// AUTOMATIC,
}

pub trait RequestContext: Debug {}

#[derive(Debug)]
pub struct RequestInput<I: Debug, C: RequestContext> {
	pub context: C,
	pub data: I,
}

pub trait ActionType<T: RequestContext, E: Debug>: PartialEq + Eq + Debug {
	fn scope() -> ActionScope;
	fn validate(&self, info: &T) -> Result<(), E>;
}

pub trait Exception<E: Debug>: Debug {
	fn handle(self) -> E;
}

pub trait Action<I, O, E, X>
where
	I: Debug,
	O: Debug,
	E: Debug,
	X: Exception<E>,
{
	fn new(input: I) -> Self;
	fn run(self) -> Result<O, X>;
}

pub trait ActionRequest<I, O, E, X>: Action<I, O, E, X>
where
	I: Debug,
	O: Debug,
	E: Debug,
	X: Exception<E>,
{
	fn request(input: Result<I, X>) -> Result<O, E>;
}

impl<I, O, E, X, T> ActionRequest<I, O, E, X> for T
where
	I: Debug,
	O: Debug,
	E: Debug,
	X: Exception<E>,
	T: Action<I, O, E, X>,
{
	fn request(input: Result<I, X>) -> Result<O, E> {
		let input = input.map_err(|err| err.handle())?;
		let action = Self::new(input);
		let action_result = action.run();
		let result = action_result.map_err(|err| err.handle());
		result
	}
}
