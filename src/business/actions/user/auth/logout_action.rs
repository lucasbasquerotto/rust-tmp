use crate::{
	business::actions::user_actions::UserAction,
	lib::core::action_core::{ActionInput, ActionResult, CoreAction, CoreActionType},
};

pub struct LogoutAction(ActionInput<()>);

impl CoreAction<(), ()> for LogoutAction {
	fn get_type() -> CoreActionType {
		Box::new(UserAction::LOGOUT)
	}

	fn new(input: ActionInput<()>) -> Self {
		Self(input)
	}

	fn run(self) -> ActionResult<()> {
		println!("logout");
		Ok(())
	}
}
