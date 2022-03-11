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

pub struct ActionInput<T> {
	pub request: T,
}

pub trait ActionCreator<'a, I, O, T: 'a + Action<O>> {
	fn new(&'a self, input: &'a ActionInput<I>) -> T;
}

pub trait GeneralActionCreator {}

pub trait ActionRequest<'a, I, O, T>: ActionCreator<'a, I, O, T>
where
	T: 'a + Action<O>,
{
	fn run(&'a self, get_input: &'a dyn Fn() -> &'a ActionInput<I>) -> GeneralActionResult<O> {
		let action = self.new(get_input());
		let action_result = action.run();
		let result = action_result.map_err(|err| err.run());
		result
	}
}

impl<'a, I, O, K, T> ActionRequest<'a, I, O, K> for T
where
	K: 'a + Action<O>,
	T: GeneralActionCreator + ActionCreator<'a, I, O, K>,
{
}
