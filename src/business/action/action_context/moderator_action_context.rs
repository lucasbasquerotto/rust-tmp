use crate::business::action::{
	action_data::moderator_action_data::{ModeratorRequestContext, ModeratorSession},
	business_action::DescriptiveRequestContext,
};

impl DescriptiveRequestContext for ModeratorRequestContext {
	fn description(&self) -> String {
		let ModeratorRequestContext {
			action_type,
			session: ModeratorSession { user_id, .. },
			..
		} = &self;
		let action_id = action_type.get_id();
		format!("action({action_id}: {action_type:?}), moderator({user_id:?})")
	}
}
