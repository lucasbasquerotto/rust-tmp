use crate::{
	business::action::{
		action_type::moderator_action_type::ModeratorActionType,
		data::moderator_action_data::ModeratorRequestContext,
		definition::business_action::{ModeratorAction, ModeratorActionResult},
	},
	lib::core::action::{RequestContext, RequestInput},
};

#[derive(Debug)]
pub struct EchoErrorAction<T: RequestContext>(RequestInput<(), T>);

impl ModeratorAction<(), ()> for EchoErrorAction<ModeratorRequestContext> {
	fn action_type() -> ModeratorActionType {
		ModeratorActionType::EchoError
	}

	fn new(input: RequestInput<(), ModeratorRequestContext>) -> ModeratorActionResult<Self> {
		Ok(Self(input))
	}

	fn run_inner(self) -> ModeratorActionResult<()> {
		error!("echo error action");
		Ok(())
	}
}
