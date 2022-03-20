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

pub trait ActionType: PartialEq + Eq {
	fn scope() -> ActionScope;
}

pub trait Action<C, I, O, E, T>
where
	C: RequestContext,
	T: ActionType,
	Self: Sized,
{
	//fn action_type() -> T;
	fn new(input: RequestInput<I, C>) -> Result<Self, E>;
	fn run(self) -> Result<O, E>;
}

// pub trait ActionRequest<C, I, O, E, T>: Action<C, I, O, E, T>
// where
// 	C: RequestContext,
// 	T: ActionType,
// {
// 	fn request(input: Result<RequestInput<I, C>, E>) -> Result<O, E>;
// }

// impl<C, I, O, E, X, T, A> ActionRequest<C, I, O, E, X, T> for A
// where
// 	C: RequestContext,
// 	X: Exception<E>,
// 	T: ActionType<C, E, X>,
// 	A: Action<C, I, O, E, X, T>,
// {
// 	fn request(input: Result<RequestInput<I, C>, X>) -> Result<O, E> {
// 		let input = input.map_err(|err| err.handle())?;
// 		let action = Self::new(input).map_err(|err| err.handle())?;
// 		let action_result = action.run();
// 		let result = action_result.map_err(|err| err.handle());
// 		result
// 	}
// }
