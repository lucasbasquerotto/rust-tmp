use crate::core::dao::db_migration;

use super::user::web_user;
use rocket::{Build, Rocket};

#[derive(FromFormField)]
enum Lang {
	#[field(value = "en")]
	English,
	#[field(value = "ru")]
	Russian,
}

#[derive(FromForm)]
struct Options<'r> {
	emoji: bool,
	name: Option<&'r str>,
}

#[get("/world")]
fn world() -> &'static str {
	"Hello, world!"
}

#[get("/мир")]
fn mir() -> &'static str {
	"Привет, мир!"
}

#[get("/<name>/<age>")]
fn wave(name: &str, age: u8) -> String {
	format!("👋 Hello, {} year old named {}!", age, name)
}

#[get("/?<lang>&<opt..>")]
fn hello(lang: Option<Lang>, opt: Options<'_>) -> String {
	let mut greeting = String::new();
	if opt.emoji {
		greeting.push_str("👋 ");
	}

	match lang {
		Some(Lang::Russian) => greeting.push_str("Привет"),
		Some(Lang::English) => greeting.push_str("Hello"),
		None => greeting.push_str("Hi"),
	}

	if let Some(name) = opt.name {
		greeting.push_str(", ");
		greeting.push_str(name);
	}

	greeting.push('!');
	greeting
}

pub fn launch_rocket() -> Rocket<Build> {
	rocket::build()
		.attach(db_migration::stage())
		.mount("/", routes![hello])
		.mount("/hello", routes![world, mir])
		.mount("/wave", routes![wave])
		.mount("/user", web_user::routes())
}

#[cfg(test)]
mod tests {
	use rocket::http::{RawStr, Status};
	use rocket::local::blocking::Client;

	#[test]
	fn hello() {
		let langs = &["", "ru", "en", "unknown"];
		let ex_lang = &["Hi", "Привет", "Hello", "Hi"];

		let emojis = &["", "on", "true", "false", "no", "yes", "off"];
		let ex_emoji = &["", "👋 ", "👋 ", "", "", "👋 ", ""];

		let names = &["", "Bob", "Bob+Smith"];
		let ex_name = &["!", ", Bob!", ", Bob Smith!"];

		let client = Client::tracked(super::launch_rocket()).unwrap();
		for n in 0..(langs.len() * emojis.len() * names.len()) {
			let i = n / (emojis.len() * names.len());
			let j = n % (emojis.len() * names.len()) / names.len();
			let k = n % (emojis.len() * names.len()) % names.len();

			let (lang, ex_lang) = (langs[i], ex_lang[i]);
			let (emoji, ex_emoji) = (emojis[j], ex_emoji[j]);
			let (name, ex_name) = (names[k], ex_name[k]);
			let expected = format!("{}{}{}", ex_emoji, ex_lang, ex_name);

			let q = |name, s: &str| match s.is_empty() {
				true => "".into(),
				false => format!("&{}={}", name, s),
			};

			let uri = format!(
				"/?{}{}{}",
				q("lang", lang),
				q("emoji", emoji),
				q("name", name)
			);
			let response = client.get(uri).dispatch();
			assert_eq!(response.into_string().unwrap(), expected);

			let uri = format!(
				"/?{}{}{}",
				q("emoji", emoji),
				q("name", name),
				q("lang", lang)
			);
			let response = client.get(uri).dispatch();
			assert_eq!(response.into_string().unwrap(), expected);
		}
	}

	#[test]
	fn hello_world() {
		let client = Client::tracked(super::launch_rocket()).unwrap();
		let response = client.get("/hello/world").dispatch();
		assert_eq!(response.into_string(), Some("Hello, world!".into()));
	}

	#[test]
	fn hello_mir() {
		let client = Client::tracked(super::launch_rocket()).unwrap();
		let response = client.get("/hello/%D0%BC%D0%B8%D1%80").dispatch();
		assert_eq!(response.into_string(), Some("Привет, мир!".into()));
	}

	#[test]
	fn wave() {
		let client = Client::tracked(super::launch_rocket()).unwrap();
		for &(name, age) in &[("Bob%20Smith", 22), ("Michael", 80), ("A", 0), ("a", 127)] {
			let uri = format!("/wave/{}/{}", name, age);
			let real_name = RawStr::new(name).percent_decode_lossy();
			let expected = format!("👋 Hello, {} year old named {}!", age, real_name);
			let response = client.get(uri).dispatch();
			assert_eq!(response.into_string().unwrap(), expected);

			for bad_age in &["1000", "-1", "bird", "?"] {
				let bad_uri = format!("/wave/{}/{}", name, bad_age);
				let response = client.get(bad_uri).dispatch();
				assert_eq!(response.status(), Status::NotFound);
			}
		}
	}
}
