use crate::{
	business::action::{
		business_action::ActionResult,
		def::user_action::UserAction,
		r#type::user_action_type::{UserActionType, UserRequestInfo},
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

	fn run(self) -> ActionResult<()> {
		println!("logout");
		Ok(())
	}
}
