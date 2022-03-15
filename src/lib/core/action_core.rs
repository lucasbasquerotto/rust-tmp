use std::fmt::Debug;

pub enum ActionContext {
	USER,
	// MODERATOR,
	// AUTOMATIC,
}

pub trait RequestInfo: Debug {}

#[derive(Debug)]
pub struct RequestInput<D: Debug, I: RequestInfo> {
	pub info: I,
	pub data: D,
}

pub trait ActionType<T: RequestInfo, E: Debug>: PartialEq + Eq + Debug {
	fn context() -> ActionContext;
	fn validate(input: T) -> Result<(), E>;
}

// #[derive(Debug)]
// pub struct CoreException {
// 	pub private: Option<ErrorData>,
// 	pub public: Option<ErrorData>,
// }

// #[derive(Debug)]
// pub struct ErrorData {
// 	pub key: &'static str,
// 	pub msg: &'static str,
// 	pub params: Option<HashMap<String, String>>,
// 	pub meta: Option<HashMap<String, String>>,
// }

// impl Exception<Option<ErrorData>> for CoreException {
// 	fn handle(self) -> Option<ErrorData> {
// 		//TODO log
// 		println!(
// 			"error: {private:?} / {public:?}",
// 			private = &self.private,
// 			public = &self.public
// 		);
// 		self.public
// 	}
// }

// pub type ActionResult<T> = Result<T, CoreException>;

// pub type ActionRequestResult<T> = Result<T, Option<ErrorData>>;

// pub trait CoreAction<I: Debug, O: Debug> {
// 	fn new(input: I) -> Self;
// 	fn run(self) -> ActionResult<O>;
// }

// impl<I, O, T> Action<I, O, Option<ErrorData>, CoreException> for T
// where
// 	I: Debug,
// 	O: Debug,
// 	T: CoreAction<I, O>,
// {
// 	fn new(input: I) -> Self {
// 		Self::new(input)
// 	}

// 	fn run(self) -> ActionResult<O> {
// 		self.run()
// 	}
// }

// pub trait CoreAction<T: ActionType<u32>, I: Debug, O: Debug> {
// 	fn get_type() -> T;
// 	fn new(input: I) -> Self;
// 	fn run(self) -> ActionResult<O>;
// }

// impl<A, I, O, T> Action<ActionInput<I>, O, Option<ErrorData>, CoreException> for CoreAction<A, I, O>
// where
// 	I: Debug,
// 	O: Debug,
// 	A: ActionType<u32>,
// {
// 	fn new(input: ActionInput<I>) -> Self {
// 		Self::new(input)
// 	}

// 	fn run(self) -> ActionResult<O> {
// 		self.run()
// 	}
// }
