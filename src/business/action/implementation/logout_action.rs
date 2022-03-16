use crate::{
	business::action::{
		action_type::user_action_type::{UserActionType, UserRequestContext},
		definition::user_action::{UserAction, UserActionResult},
	},
	lib::core::action::{RequestContext, RequestInput},
};

#[derive(Debug)]
pub struct LogoutAction<T: RequestContext>(RequestInput<(), T>);

impl UserAction<(), ()> for LogoutAction<UserRequestContext> {
	fn action_type() -> UserActionType {
		UserActionType::LOGOUT
	}

	fn new(input: RequestInput<(), UserRequestContext>) -> Self {
		Self(input)
	}

	fn input(&self) -> &RequestInput<(), UserRequestContext> {
		&self.0
	}

	fn run_inner(self) -> UserActionResult<()> {
		println!("logout");
		Ok(())
	}
}
