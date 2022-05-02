use crate::{
	business::action::user::{register_user_action, select_user_action},
	core::{
		action::data::action_data::AuthBasicContext,
		web::definition::web_action::{WebAction, WebActionResult},
	},
	shared::data::user_data::UserId,
};
use rocket::serde::json::Json;

#[post("/", data = "<input>")]
async fn register_user(
	context: AuthBasicContext,
	input: Json<register_user_action::Input>,
) -> WebActionResult<register_user_action::Output> {
	register_user_action::Action::request(context.data(input.0)).await
}

#[get("/<id>")]
async fn select_user(
	context: AuthBasicContext,
	id: u64,
) -> WebActionResult<select_user_action::Output> {
	let input = context.data(select_user_action::Input(UserId(id)));
	select_user_action::Action::request(input).await
}

pub fn routes() -> Vec<rocket::Route> {
	routes![register_user, select_user]
}

#[cfg(test)]
mod tests {
	use crate::business::action::user::{register_user_action, select_user_action};
	use rocket::{http::Status, local::blocking::Client};

	fn get_client() -> Client {
		Client::tracked(rocket::build().mount("/user", super::routes())).unwrap()
	}

	#[test]
	fn register_user_ok() {
		let client = get_client();

		let input = register_user_action::Input {
			email: "a@b.com".into(),
			name: "User 01".into(),
			pass: "p4$$w0rd".into(),
		};
		let input_json = rocket::serde::json::serde_json::to_string(&input).unwrap();
		let register_user_action::tests::ActionMock { output, mocks: _m } =
			register_user_action::tests::mock_action(input);
		let response = client.post("/user").body(input_json).dispatch();

		assert_eq!(response.status(), Status::Ok);
		assert_eq!(response.into_json(), Some(output));
	}

	#[test]
	fn register_user_error() {
		let client = get_client();

		let output: Option<register_user_action::Output> = None;

		let response = client.post("/user").dispatch();

		assert_eq!(response.status(), Status::BadRequest);
		assert_eq!(response.into_json(), output);

		let input = register_user_action::Input {
			email: "a@b.com".into(),
			name: "User 01".into(),
			pass: "p4$$w0rd".into(),
		};
		let input_json = rocket::serde::json::serde_json::to_string(&input).unwrap();

		let response = client.post("/user").body(input_json).dispatch();

		assert_eq!(response.status(), Status::InternalServerError);
		assert_eq!(response.into_json(), output);
	}

	#[test]
	fn select_user_ok() {
		let client = get_client();

		let select_user_action::tests::ActionMock {
			user_id,
			output,
			mocks: _m,
		} = select_user_action::tests::mock_action(123);
		let user_id = user_id.0;
		let uri = format!("/user/{user_id}");
		let response = client.get(uri).dispatch();

		assert_eq!(response.status(), Status::Ok);
		assert_eq!(response.into_json(), Some(output));
	}

	#[test]
	fn select_user_error() {
		let client = get_client();

		let user_id = 123;
		let uri = format!("/user/{user_id}");
		let response = client.get(uri).dispatch();

		let output: Option<select_user_action::Output> = None;

		assert_eq!(response.status(), Status::InternalServerError);
		assert_eq!(response.into_json(), output);
	}
}
