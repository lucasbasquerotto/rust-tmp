use crate::core::action::{
	action_type::moderator_action_type::ModeratorActionType,
	data::action_data::{ActionErrorInfo, ActionResultInfo, RequestContext, RequestInput},
};

use super::action_data::{Application, Request, Session};

////////////////////////////////////////////////
//////////////////// INPUT /////////////////////
////////////////////////////////////////////////

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModeratorSession {
	pub admin: bool,
	pub user_id: u64,
	pub allowed_actions: Vec<ModeratorActionType>,
}

impl Session for ModeratorSession {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModeratorRequestContext {
	pub application: Application,
	pub session: ModeratorSession,
	pub request: Request,
}

impl RequestContext for ModeratorRequestContext {}

pub type ModeratorRequestInput<I> = RequestInput<I, ModeratorRequestContext>;

pub type ModeratorActionInput<I> = Result<ModeratorRequestInput<I>, ModeratorActionError>;

pub type ModeratorOutputInfo<D> = ActionResultInfo<ModeratorActionType, ModeratorRequestContext, D>;

pub type ModeratorErrorInfo<E> = ActionErrorInfo<ModeratorActionType, ModeratorRequestContext, E>;

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq)]
pub enum ModeratorActionError {
	NotAllowed(ModeratorActionType),
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {
	use crate::core::action::action_type::moderator_action_type::ModeratorActionType;
	use crate::core::action::data::action_data::tests::{ApplicationBuilder, RequestBuilder};
	use crate::core::action::data::action_data::{Application, Request};

	use super::{ModeratorRequestContext, ModeratorSession};

	#[allow(dead_code)]
	pub struct ModeratorSessionBuilder(ModeratorSession);

	#[allow(dead_code)]
	impl ModeratorSessionBuilder {
		pub fn new() -> Self {
			Self(ModeratorSession {
				admin: false,
				user_id: 0,
				allowed_actions: vec![],
			})
		}

		pub fn admin(mut self, admin: bool) -> Self {
			self.0.admin = admin;
			self
		}

		pub fn user_id(mut self, user_id: u64) -> Self {
			self.0.user_id = user_id;
			self
		}

		pub fn allowed_actions(mut self, allowed_actions: Vec<ModeratorActionType>) -> Self {
			self.0.allowed_actions = allowed_actions;
			self
		}

		pub fn build(self) -> ModeratorSession {
			self.0
		}
	}

	#[allow(dead_code)]
	pub struct ModeratorRequestContextBuilder(ModeratorRequestContext);

	#[allow(dead_code)]
	impl ModeratorRequestContextBuilder {
		pub fn new() -> Self {
			Self(ModeratorRequestContext {
				application: ApplicationBuilder::new().build(),
				session: ModeratorSessionBuilder::new().build(),
				request: RequestBuilder::new().build(),
			})
		}

		pub fn application(mut self, application: Application) -> Self {
			self.0.application = application;
			self
		}

		pub fn session(mut self, session: ModeratorSession) -> Self {
			self.0.session = session;
			self
		}

		pub fn request(mut self, request: Request) -> Self {
			self.0.request = request;
			self
		}

		pub fn build(self) -> ModeratorRequestContext {
			self.0
		}

		pub fn build_admin() -> ModeratorRequestContext {
			Self::new()
				.session(ModeratorSessionBuilder::new().admin(true).build())
				.build()
		}
	}
}
