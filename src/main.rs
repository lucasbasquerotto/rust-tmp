use web::web_root::launch_rocket;

extern crate chrono;
extern crate log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;
extern crate strum;

mod business;
mod core;
mod external;
mod lib;
mod shared;
mod web;

#[launch]
fn rocket() -> _ {
	launch_rocket()
}

#[cfg(test)]
pub mod tests {
	pub mod test_utils;

	use self::test_utils::tests;

	#[ctor::ctor]
	fn init() {
		tests::init();
	}

	#[tokio::test]
	async fn main() {}
}
