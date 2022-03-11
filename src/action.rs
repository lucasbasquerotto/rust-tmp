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

pub type ActionInput<T> = ActionResult<T>;

pub trait Action<O> {
	fn run(self) -> ActionResult<O>;
}

pub trait GeneralActionOutput: std::fmt::Debug {}

impl GeneralActionOutput for ErrorData {}

pub type GeneralActionMainResult = Box<dyn GeneralActionOutput>;

pub type GeneralActionResult = ActionResult<GeneralActionMainResult>;

pub trait Cast<T> {
	fn cast(self) -> T;
}

impl<T: 'static + GeneralActionOutput> Cast<GeneralActionResult> for ActionResult<T> {
	fn cast(self) -> GeneralActionResult {
		self.map(|data| Box::new(data) as _)
	}
}

pub trait GeneralAction: Action<GeneralActionMainResult> {
	fn exec(self) -> GeneralActionMainResult;
}

// type ActionInputContext<T> = (a: T);

impl<T: Action<GeneralActionMainResult>> GeneralAction for T {
	fn exec(self) -> GeneralActionMainResult {
		let result = self.run();
		result.unwrap_or_else(|err| Box::new(err.run()))
	}
}
