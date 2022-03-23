#[macro_use]
extern crate log;

mod business;

pub fn main() {}

#[cfg(test)]
pub mod tests {
	pub mod test_utils;

	#[test]
	pub fn main() {
		// log::set_logger(&MY_LOGGER).unwrap();
		// log::set_max_level(LevelFilter::Info);
	}
}
