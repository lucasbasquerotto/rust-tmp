use std::fmt::Debug;

pub enum ActionScope {
	User,
	Moderator,
	// AUTOMATIC,
}

pub trait RequestContext: Debug {}

#[derive(Debug)]
pub struct RequestInput<I: Debug, C: RequestContext> {
	pub context: C,
	pub data: I,
}

pub trait ActionType<C, E, X, D>: PartialEq + Eq + Debug
where
	C: RequestContext,
	E: Debug,
	X: Exception<E>,
	D: Debug + Eq + PartialEq,
{
	fn scope() -> ActionScope;
	fn id(&self) -> D;
	fn validate(&self, info: &C) -> Result<(), X>;
}

pub trait Exception<E: Debug>: Debug {
	fn handle(self) -> E;
}

pub trait Action<C, I, O, E, X, D, T>
where
	C: RequestContext,
	I: Debug,
	O: Debug,
	E: Debug,
	X: Exception<E>,
	D: Debug + Eq + PartialEq,
	T: ActionType<C, E, X, D>,
{
	fn action_type() -> T;
	fn new(input: RequestInput<I, C>) -> Self;
	fn input(&self) -> &RequestInput<I, C>;
	fn run(self) -> Result<O, X>;
}

pub trait ActionRequest<C, I, O, E, X, D, T>: Action<C, I, O, E, X, D, T>
where
	C: RequestContext,
	I: Debug,
	O: Debug,
	E: Debug,
	X: Exception<E>,
	D: Debug + Eq + PartialEq,
	T: ActionType<C, E, X, D>,
{
	fn request(input: Result<RequestInput<I, C>, X>) -> Result<O, E>;
}

impl<C, I, O, E, X, D, T, A> ActionRequest<C, I, O, E, X, D, T> for A
where
	C: RequestContext,
	I: Debug,
	O: Debug,
	E: Debug,
	X: Exception<E>,
	D: Debug + Eq + PartialEq,
	T: ActionType<C, E, X, D>,
	A: Action<C, I, O, E, X, D, T>,
{
	fn request(input: Result<RequestInput<I, C>, X>) -> Result<O, E> {
		let input = input.map_err(|err| err.handle())?;
		let action = Self::new(input);
		let action_result = action.run();
		let result = action_result.map_err(|err| err.handle());
		result
	}
}
