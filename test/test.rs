trait Bar<T> {
	fn with_parameter(&self, arg: u32, value: String) -> String;
	fn test(value: T) -> T;
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

fn test_int<T: Bar<u32>>(b: &T) {
	b.with_parameter(20, "inner - int".to_owned());
}

fn test_str<T: Bar<String>>(b: &T) {
	b.with_parameter(30, "inner - str".to_owned());
}

fn main() {
	let test = Baz();
	test_int(&test);
	test_str(&test);
	Bar::<u32>::with_parameter(&test, 2, "outer - int".to_owned());
	Bar::<String>::with_parameter(&test, 3, "outer - str".to_owned());
}
