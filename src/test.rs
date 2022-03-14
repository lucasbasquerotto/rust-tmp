trait Foo<P, T> {
	fn new(arg: u32) -> String;
}

trait Bar<T> {
	fn with_parameter(arg: u32, value: String) -> String;
	fn test(value: T) -> T;
}

impl<T: Bar<P>, P: Default> Foo<P, T> for T {
	fn new(arg: u32) -> String {
		Self::with_parameter(arg, "aaa".to_owned())
	}
}

struct Baz();

impl Bar<u32> for Baz {
	fn with_parameter(arg: u32, value: String) -> String {
		println!("u32: arg: {arg}, value: {value}");
		value
	}

	fn test(value: u32) -> u32 {
		todo!()
	}
}

impl Bar<String> for Baz {
	fn with_parameter(arg: u32, value: String) -> String {
		println!("String: arg: {arg}, value: {value}");
		value
	}

	fn test(value: String) -> String {
		todo!()
	}
}

fn test() {
	Baz::with_parameter(10, "value1".to_owned());
}
