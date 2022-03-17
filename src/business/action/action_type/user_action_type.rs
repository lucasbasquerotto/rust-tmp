use crate::{
	business::action::action_data::{
		general_action_data::{BusinessException, ErrorData},
		user_action_data::{UserActionType, UserRequestContext},
	},
	lib::core::action::{ActionScope, ActionType},
};

impl ActionType<UserRequestContext, Option<ErrorData>, BusinessException<UserRequestContext>, u32>
	for UserActionType
{
	fn scope() -> ActionScope {
		ActionScope::USER
	}

	fn id(&self) -> u32 {
		match self {
			UserActionType::LOGIN => 1,
			UserActionType::LOGOUT => 2,
		}
	}

	fn validate(
		&self,
		context: &UserRequestContext,
	) -> Result<(), BusinessException<UserRequestContext>> {
		match self {
			UserActionType::LOGIN => context.to_no_auth().map(|_| ()),
			UserActionType::LOGOUT => context.to_auth().map(|_| ()),
		}
	}
}

// #[derive(Debug)]
// enum UserTypeError {
// 	UNAUTHENTICATED,
// 	AUTHENTICATED,
// }

// impl BusinessErrorGenerator<UserRequestContext> for UserTypeError {
// 	fn private_error(&self) -> Option<ErrorData> {
// 		match self {
// 			UserTypeError::UNAUTHENTICATED => None,
// 			UserTypeError::AUTHENTICATED => None,
// 		}
// 	}

// 	fn public_error(&self) -> Option<ErrorData> {
// 		match self {
// 			UserTypeError::UNAUTHENTICATED => {
// 				self.error_msg("You must be authenticated to execute this action.")
// 			}
// 			UserTypeError::AUTHENTICATED => {
// 				self.error_msg("You can't execute this action while authenticated.")
// 			}
// 		}
// 	}
// }

// fn validate_auth(
// 	context: &UserRequestContext,
// 	expect_auth: bool,
// ) -> Result<(), BusinessException<UserRequestContext>> {
// 	match context.session.user_id {
// 		Some(_) => {
// 			if expect_auth {
// 				Ok(())
// 			} else {
// 				Err(UserTypeError::AUTHENTICATED.exception(context))
// 			}
// 		}
// 		None => {
// 			if expect_auth {
// 				Err(UserTypeError::UNAUTHENTICATED.exception(context))
// 			} else {
// 				Ok(())
// 			}
// 		}
// 	}
// }
