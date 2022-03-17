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
		ActionScope::User
	}

	fn id(&self) -> u32 {
		self.get_id()
	}

	fn validate(
		&self,
		context: &UserRequestContext,
	) -> Result<(), BusinessException<UserRequestContext>> {
		match self {
			UserActionType::Login => context.to_no_auth().map(|_| ()),
			UserActionType::Logout => context.to_auth().map(|_| ()),
		}
	}
}
