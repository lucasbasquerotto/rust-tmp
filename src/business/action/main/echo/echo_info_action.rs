use crate::{
	business::action::{
		action_type::moderator_action_type::ModeratorActionType,
		data::moderator_action_data::ModeratorRequestContext,
		definition::business_action::{ModeratorAction, ModeratorActionResult},
	},
	lib::core::action::{RequestContext, RequestInput},
};

#[derive(Debug)]
pub struct EchoInfoAction<T: RequestContext>(RequestInput<(), T>);

impl ModeratorAction<(), ()> for EchoInfoAction<ModeratorRequestContext> {
	fn action_type() -> ModeratorActionType {
		ModeratorActionType::EchoInfo
	}

	fn new(input: RequestInput<(), ModeratorRequestContext>) -> ModeratorActionResult<Self> {
		Ok(Self(input))
	}

	fn run_inner(self) -> ModeratorActionResult<()> {
		info!("echo info action");
		Ok(())
	}
}
