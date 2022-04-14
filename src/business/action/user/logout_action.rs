use crate::core::action::definition::action::{ActionError, UserAction};
use crate::{
	core::action::{
		action_type::user_action_type::UserActionType,
		data::{
			action_data::{DescriptiveError, ErrorData},
			user_action_data::{UserActionError, UserRequestInput},
		},
	},
	lib::data::result::AsyncResult,
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
	fn private_error(&self) -> Option<DescriptiveError> {
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

impl From<UserActionError> for Error {
	fn from(error: UserActionError) -> Self {
		Self::UserError(error)
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

	fn new(input: UserRequestInput<()>) -> AsyncResult<Self, Error> {
		Box::pin(async { Ok(Self(input)) })
	}

	fn run_inner(self) -> AsyncResult<(), Error> {
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
	use crate::core::action::data::{
		action_data::{ActionContext, RequestInput},
		user_action_data::{tests::UserRequestContextBuilder, UserOutputInfo},
	};
	use crate::{core::action::definition::action::Action, tests::test_utils::tests::run_test};

	#[tokio::test]
	async fn main() {
		run_test(|_| async {
			let context = UserRequestContextBuilder::build_no_auth();
			let action_context = ActionContext {
				action_type: super::USER_ACTION_TYPE,
				context: Some(context.clone()),
			};
			let result = super::Action::run(Ok(RequestInput { data: (), context })).await;
			assert_eq!(
				&result,
				&Ok(UserOutputInfo {
					action_context,
					data: (),
				}),
			);
		})
		.await;
	}
}
