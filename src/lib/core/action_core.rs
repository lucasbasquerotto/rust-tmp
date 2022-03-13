use crate::lib::base::action::{Action, Exception};
use std::{collections::HashMap, fmt::Debug};

pub trait ActionType<T> {
	fn id(&self) -> T;
}

#[derive(Debug)]
pub struct CoreException {
	pub private: Option<ErrorData>,
	pub public: Option<ErrorData>,
}

impl Exception<Option<ErrorData>> for CoreException {
	fn handle(self) -> Option<ErrorData> {
		//TODO log
		println!(
			"error: {private:?} / {public:?}",
			private = &self.private,
			public = &self.public
		);
		self.public
	}
}

#[derive(Debug)]
pub struct ErrorData {
	pub key: &'static str,
	pub msg: &'static str,
	pub params: Option<HashMap<String, String>>,
	pub meta: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct ActionInput<T: Debug> {
	pub request: T,
}

pub type ActionResult<T> = Result<T, CoreException>;

pub type ActionRequestResult<T> = Result<T, Option<ErrorData>>;

pub type CoreActionType = Box<dyn ActionType<u32>>;

pub trait CoreAction<I: Debug, O: Debug> {
	fn get_type() -> CoreActionType;
	fn new(input: ActionInput<I>) -> Self;
	fn run(self) -> ActionResult<O>;
}

impl<I, O, T> Action<ActionInput<I>, O, Option<ErrorData>, CoreException> for T
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
