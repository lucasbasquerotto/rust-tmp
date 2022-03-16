use crate::{
	business::action::{
		action_type::user_action_type::{UserActionType, UserRequestInfo},
		definition::user_action::{UserAction, UserActionResult},
	},
	lib::core::action_core::{RequestInfo, RequestInput},
};

#[derive(Debug)]
pub struct LogoutAction<T: RequestInfo>(RequestInput<(), T>);

impl UserAction<(), ()> for LogoutAction<UserRequestInfo> {
	fn action_type() -> UserActionType {
		UserActionType::LOGOUT
	}

	fn new(input: RequestInput<(), UserRequestInfo>) -> Self {
		Self(input)
	}

	fn input(&self) -> &RequestInput<(), UserRequestInfo> {
		&self.0
	}

	fn run(self) -> UserActionResult<()> {
		println!("logout");
		Ok(())
	}
}
