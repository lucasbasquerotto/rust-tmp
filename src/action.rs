use std::fmt::Debug;

pub trait Identifiable<T> {
	fn id(&self) -> T;
}

#[derive(Debug)]
pub struct ErrorData {
	pub key: &'static str,
	pub msg: &'static str,
	pub params: Option<std::collections::HashMap<String, String>>,
}

pub trait Exception: std::fmt::Debug {
	fn run(&self) -> ErrorData;
}

pub type ActionResult<T> = Result<T, Box<dyn Exception>>;

pub trait Action<T> {
	fn run(self) -> ActionResult<T>;
}

pub trait GeneralActionOutput: std::fmt::Debug {}

impl GeneralActionOutput for ErrorData {}

pub type GeneralActionResult<T> = Result<T, ErrorData>;

pub trait ActionResultGenerator<T> {
	fn result(self) -> T;
}

#[derive(Debug)]
pub struct ActionInput<T: Debug> {
	pub request: T,
}

pub trait ActionCreator<I, O, T>
where
	I: Debug,
	O: Debug,
	T: Action<O>,
{
	fn new(input: ActionInput<I>) -> T;
}

pub trait GeneralActionCreator {}

pub trait ActionRequest<I, O, T>: ActionCreator<I, O, T>
where
	I: Debug,
	O: Debug,
	T: Action<O>,
{
	fn run<F: Fn() -> ActionInput<I>>(get_input: F) -> GeneralActionResult<O> {
		let input = get_input();
		let action = Self::new(input);
		let action_result = action.run();
		let result = action_result.map_err(|err| err.run());
		result
	}
}

impl<I, O, K, T> ActionRequest<I, O, K> for T
where
	I: Debug,
	O: Debug,
	K: Action<O>,
	T: ActionCreator<I, O, K>,
{
}
