use std::fmt::Debug;

pub trait DescriptiveRequestContext: Debug + Clone {
	fn description(&self) -> String;
}

pub trait ActionLogger {
	fn info(&self);
	fn warn(&self);
	fn error(&self);
	fn debug(&self);
}
