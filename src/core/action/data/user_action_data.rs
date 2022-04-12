use chrono::{DateTime, Utc};

use super::action_data::{Application, Request, Session};
use crate::{
	core::action::{
		action_type::user_action_type::UserActionType,
		data::action_data::{ActionErrorInfo, ActionResultInfo, RequestContext, RequestInput},
	},
	lib::data::result::AsyncResult,
};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UserSession {
	Auth(UserAuthSession),
	NoAuth(UserNoAuthSession),
	Unconfirmed(UserUnconfirmedSession),
}

impl Session for UserSession {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserAuthSession {
	pub created_at: DateTime<Utc>,
	pub user_id: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserUnconfirmedSession {
	pub created_at: DateTime<Utc>,
	pub user_id: u64,
}

impl Session for UserAuthSession {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserNoAuthSession {
	pub created_at: DateTime<Utc>,
}

impl Session for UserNoAuthSession {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserRequestContext {
	pub application: Application,
	pub session: UserSession,
	pub request: Request,
}

impl RequestContext for UserRequestContext {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserAuthRequestContext {
	pub application: Application,
	pub session: UserAuthSession,
	pub request: Request,
}

impl RequestContext for UserAuthRequestContext {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserNoAuthRequestContext {
	pub application: Application,
	pub session: UserNoAuthSession,
	pub request: Request,
}

impl RequestContext for UserNoAuthRequestContext {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserUnconfirmedRequestContext {
	pub application: Application,
	pub session: UserUnconfirmedSession,
	pub request: Request,
}

impl RequestContext for UserUnconfirmedRequestContext {}

pub type UserRequestInput<I> = RequestInput<I, UserRequestContext>;

#[allow(dead_code)]
pub type UserAuthRequestInput<I> = RequestInput<I, UserAuthRequestContext>;

pub type UserNoAuthRequestInput<I> = RequestInput<I, UserNoAuthRequestContext>;

pub type UserActionInput<I> = AsyncResult<UserRequestInput<I>, UserActionError>;

pub type UserOutputInfo<D> = ActionResultInfo<UserActionType, UserRequestContext, D>;

pub type UserErrorInfo<E> = ActionErrorInfo<UserActionType, UserRequestContext, E>;

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq)]
pub enum UserActionError {
	Authenticated,
	Unauthenticated,
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use chrono::{DateTime, Utc};

	use crate::core::action::data::action_data::tests::{ApplicationBuilder, RequestBuilder};
	use crate::core::action::data::user_action_data::{
		UserNoAuthSession, UserRequestContext, UserSession,
	};
	use crate::core::action::data::{
		action_data::{Application, Request},
		user_action_data::UserAuthSession,
	};

	use super::UserUnconfirmedSession;

	#[allow(dead_code)]
	pub struct UserNoAuthSessionBuilder(UserNoAuthSession);

	#[allow(dead_code)]
	impl UserNoAuthSessionBuilder {
		pub fn new() -> Self {
			Self(UserNoAuthSession {
				created_at: Utc::now(),
			})
		}

		pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
			self.0.created_at = created_at;
			self
		}

		pub fn build(self) -> UserNoAuthSession {
			self.0
		}
	}

	#[allow(dead_code)]
	pub struct UserAuthSessionBuilder(UserAuthSession);

	#[allow(dead_code)]
	impl UserAuthSessionBuilder {
		pub fn new() -> Self {
			Self(UserAuthSession {
				created_at: Utc::now(),
				user_id: 0,
			})
		}

		pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
			self.0.created_at = created_at;
			self
		}

		pub fn user_id(mut self, user_id: u64) -> Self {
			self.0.user_id = user_id;
			self
		}

		pub fn build(self) -> UserAuthSession {
			self.0
		}
	}

	#[allow(dead_code)]
	pub struct UserUnconfirmedSessionBuilder(UserUnconfirmedSession);

	#[allow(dead_code)]
	impl UserUnconfirmedSessionBuilder {
		pub fn new() -> Self {
			Self(UserUnconfirmedSession {
				created_at: Utc::now(),
				user_id: 0,
			})
		}

		pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
			self.0.created_at = created_at;
			self
		}

		pub fn user_id(mut self, user_id: u64) -> Self {
			self.0.user_id = user_id;
			self
		}

		pub fn build(self) -> UserUnconfirmedSession {
			self.0
		}
	}

	#[allow(dead_code)]
	pub struct UserRequestContextBuilder(UserRequestContext);

	#[allow(dead_code)]
	impl UserRequestContextBuilder {
		pub fn new() -> Self {
			Self(UserRequestContext {
				application: ApplicationBuilder::new().build(),
				session: UserSession::NoAuth(UserNoAuthSessionBuilder::new().build()),
				request: RequestBuilder::new().build(),
			})
		}

		pub fn application(mut self, application: Application) -> Self {
			self.0.application = application;
			self
		}

		pub fn session(mut self, session: UserSession) -> Self {
			self.0.session = session;
			self
		}

		pub fn request(mut self, request: Request) -> Self {
			self.0.request = request;
			self
		}

		pub fn build(self) -> UserRequestContext {
			self.0
		}

		pub fn build_no_auth() -> UserRequestContext {
			Self::new().build()
		}

		pub fn build_auth() -> UserRequestContext {
			Self::new()
				.session(UserSession::Auth(UserAuthSessionBuilder::new().build()))
				.build()
		}

		pub fn build_unconfirmed() -> UserRequestContext {
			Self::new()
				.session(UserSession::Unconfirmed(
					UserUnconfirmedSessionBuilder::new().build(),
				))
				.build()
		}
	}
}
