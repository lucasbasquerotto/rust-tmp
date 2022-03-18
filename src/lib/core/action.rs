pub enum ActionScope {
	User,
	Moderator,
	// AUTOMATIC,
}

pub trait RequestContext {}

#[derive(Debug)]
pub struct RequestInput<I, C: RequestContext> {
	pub context: C,
	pub data: I,
}

pub trait ActionType<C, E, X, D>: PartialEq + Eq
where
	C: RequestContext,
	X: Exception<E>,
	D: Eq + PartialEq,
{
	fn scope() -> ActionScope;
	fn id(&self) -> D;
}

pub trait Exception<E> {
	fn handle(self) -> E;
}

pub trait Action<C, I, O, E, X, D, T>
where
	C: RequestContext,
	X: Exception<E>,
	D: Eq + PartialEq,
	T: ActionType<C, E, X, D>,
	Self: Sized,
{
	fn action_type() -> T;
	fn new(input: RequestInput<I, C>) -> Result<Self, X>;
	fn run(self) -> Result<O, X>;
}

pub trait ActionRequest<C, I, O, E, X, D, T>: Action<C, I, O, E, X, D, T>
where
	C: RequestContext,
	X: Exception<E>,
	D: Eq + PartialEq,
	T: ActionType<C, E, X, D>,
{
	fn request(input: Result<RequestInput<I, C>, X>) -> Result<O, E>;
}

impl<C, I, O, E, X, D, T, A> ActionRequest<C, I, O, E, X, D, T> for A
where
	C: RequestContext,
	X: Exception<E>,
	D: Eq + PartialEq,
	T: ActionType<C, E, X, D>,
	A: Action<C, I, O, E, X, D, T>,
{
	fn request(input: Result<RequestInput<I, C>, X>) -> Result<O, E> {
		let input = input.map_err(|err| err.handle())?;
		let action = Self::new(input).map_err(|err| err.handle())?;
		let action_result = action.run();
		let result = action_result.map_err(|err| err.handle());
		result
	}
}
