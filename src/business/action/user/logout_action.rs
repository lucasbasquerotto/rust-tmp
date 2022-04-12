use crate::core::action::{
	action_type::user_action_type::UserActionType,
	data::{
		action_data::{DescriptiveError, ErrorData},
		user_action_data::{UserActionError, UserRequestInput},
	},
	definition::action::ActionResult,
};
use crate::core::action::{
	data::user_action_data::UserActionInput,
	definition::action::{ActionError, UserAction},
};

////////////////////////////////////////////////
///////////////////// TYPE /////////////////////
////////////////////////////////////////////////

const USER_ACTION_TYPE: UserActionType = UserActionType::Logout;

////////////////////////////////////////////////
//////////////////// ERROR /////////////////////
////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
pub enum Error {
	UserError(UserActionError),
}

impl ActionError for Error {
	fn private_error(&self) -> DescriptiveError {
		match self {
			Error::UserError(error) => error.private_error(),
		}
	}

	fn public_error(&self) -> Option<ErrorData> {
		match self {
			Error::UserError(error) => error.public_error(),
		}
	}
}

////////////////////////////////////////////////
/////////////////// ACTION /////////////////////
////////////////////////////////////////////////

#[derive(Debug)]
pub struct Action(UserRequestInput<()>);

impl UserAction<(), (), Error> for Action {
	fn action_type() -> UserActionType {
		USER_ACTION_TYPE
	}

	fn new(input: UserActionInput<()>) -> ActionResult<Self, Error> {
		Box::pin(async { input.await.map(Self).map_err(Error::UserError) })
	}

	fn run_inner(self) -> ActionResult<(), Error> {
		Box::pin(async {
			println!("TODO: logout");
			Ok(())
		})
	}
}

////////////////////////////////////////////////
//////////////////// TESTS /////////////////////
////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use futures::executor::block_on;

	use crate::core::action::data::{
		action_data::{ActionContext, RequestInput},
		user_action_data::{tests::UserRequestContextBuilder, UserOutputInfo},
	};
	use crate::{core::action::definition::action::Action, tests::test_utils::tests::run_test};

	#[test]
	fn main() {
		run_test(|_| {
			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: super::USER_ACTION_TYPE,
				context: context.clone(),
			};
			let result = block_on(super::Action::run(RequestInput { data: (), context }));
			assert_eq!(
				&result,
				&Ok(UserOutputInfo {
					action_context,
					data: (),
				}),
			);
		});
	}
}
