extern crate chrono;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
extern crate strum;

mod business;

pub fn main() {}

#[cfg(test)]
pub mod tests {
	pub mod test_utils;

	use self::test_utils::tests;

	#[ctor::ctor]
	fn init() {
		tests::init();
	}

	#[test]
	pub fn main() {}
}
