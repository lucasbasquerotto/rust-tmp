use std::fmt::Debug;

trait Foo<P> {
	fn new(&self, arg: u32, val: P) -> String;
}

trait Bar<T> {
	fn with_parameter(&self, arg: u32, value: String) -> String;
	fn test(value: T) -> T;
}

impl<T: Bar<P>, P: Debug> Foo<P> for T {
	fn new(&self, arg: u32, val: P) -> String {
		println!("val: {val:?}");
		self.with_parameter(arg, "aaa".to_owned())
	}
}

struct Baz();

impl Bar<u32> for Baz {
	fn with_parameter(&self, arg: u32, value: String) -> String {
		println!("u32: arg: {arg}, value: {value}");
		value
	}

	fn test(value: u32) -> u32 {
		value
	}
}

impl Bar<String> for Baz {
	fn with_parameter(&self, arg: u32, value: String) -> String {
		println!("String: arg: {arg}, value: {value}");
		value
	}

	fn test(value: String) -> String {
		value
	}
}

fn main() {
	// let test = Baz();
	// test.with_parameter(10, "value1".to_owned());
}
