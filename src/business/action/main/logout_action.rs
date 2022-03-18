use crate::{
	business::action::{
		action_type::user_action_type::UserActionType,
		data::user_action_data::UserRequestContext,
		definition::business_action::{UserAction, UserActionResult},
	},
	lib::core::action::{RequestContext, RequestInput},
};

#[derive(Debug)]
pub struct LogoutAction<T: RequestContext>(RequestInput<(), T>);

impl UserAction<(), ()> for LogoutAction<UserRequestContext> {
	fn action_type() -> UserActionType {
		UserActionType::Logout
	}

	fn new(input: RequestInput<(), UserRequestContext>) -> UserActionResult<Self> {
		Ok(Self(input))
	}

	fn run_inner(self) -> UserActionResult<()> {
		println!("logout");
		Ok(())
	}
}
