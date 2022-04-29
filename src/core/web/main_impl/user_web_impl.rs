use crate::core::action::data::action_data::AuthBasicContext;
use rocket::request::{self, FromRequest};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthBasicContext {
	type Error = ();

	async fn from_request(req: &'r request::Request<'_>) -> request::Outcome<Self, Self::Error> {
		request::Outcome::Success(AuthBasicContext {
			token: req
				.headers()
				.get("auth")
				.next()
				.map(|value| value.to_string()),
		})
	}
}
