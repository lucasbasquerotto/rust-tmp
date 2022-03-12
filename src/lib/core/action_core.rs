use crate::lib::base::action::{Action, Exception};
use std::{collections::HashMap, fmt::Debug};

pub trait ActionType<T> {
	fn id(&self) -> T;
}

#[derive(Debug)]
pub struct ErrorData {
	pub key: &'static str,
	pub msg: &'static str,
	pub params: Option<HashMap<String, String>>,
	pub meta: Option<HashMap<String, String>>,
}

impl Exception<ErrorData> for ErrorData {
	fn handle(self) -> Self {
		self
	}
}

#[derive(Debug)]
pub struct ActionInput<T: Debug> {
	pub request: T,
}

pub type ActionResult<T> = Result<T, ErrorData>;

pub type CoreActionType = Box<dyn ActionType<u32>>;

pub trait CoreAction<I: Debug, O: Debug> {
	fn new(input: ActionInput<I>) -> Self;
	fn get_type(&self) -> CoreActionType;
	fn run(self) -> ActionResult<O>;
}

impl<I, O, T> Action<ActionInput<I>, O, ErrorData, ErrorData> for T
where
	I: Debug,
	O: Debug,
	T: CoreAction<I, O>,
{
	fn new(input: ActionInput<I>) -> Self {
		Self::new(input)
	}

	fn run(self) -> ActionResult<O> {
		self.run()
	}
}
