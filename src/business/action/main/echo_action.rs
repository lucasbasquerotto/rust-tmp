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

#[derive(Debug)]
pub struct EchoWarnAction<T: RequestContext>(RequestInput<(), T>);

#[derive(Debug)]
pub struct EchoErrorAction<T: RequestContext>(RequestInput<(), T>);

impl ModeratorAction<(), ()> for EchoInfoAction<ModeratorRequestContext> {
	fn action_type() -> ModeratorActionType {
		ModeratorActionType::EchoInfo
	}

	fn new(input: RequestInput<(), ModeratorRequestContext>) -> Self {
		Self(input)
	}

	fn input(&self) -> &RequestInput<(), ModeratorRequestContext> {
		&self.0
	}

	fn run_inner(self) -> ModeratorActionResult<()> {
		info!("echo info action");
		Ok(())
	}
}

impl ModeratorAction<(), ()> for EchoWarnAction<ModeratorRequestContext> {
	fn action_type() -> ModeratorActionType {
		ModeratorActionType::EchoWarn
	}

	fn new(input: RequestInput<(), ModeratorRequestContext>) -> Self {
		Self(input)
	}

	fn input(&self) -> &RequestInput<(), ModeratorRequestContext> {
		&self.0
	}

	fn run_inner(self) -> ModeratorActionResult<()> {
		warn!("echo warn action");
		Ok(())
	}
}

impl ModeratorAction<(), ()> for EchoErrorAction<ModeratorRequestContext> {
	fn action_type() -> ModeratorActionType {
		ModeratorActionType::EchoError
	}

	fn new(input: RequestInput<(), ModeratorRequestContext>) -> Self {
		Self(input)
	}

	fn input(&self) -> &RequestInput<(), ModeratorRequestContext> {
		&self.0
	}

	fn run_inner(self) -> ModeratorActionResult<()> {
		error!("echo error action");
		Ok(())
	}
}
