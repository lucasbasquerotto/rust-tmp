mod action;
mod specific;

use crate::action::GeneralAction;
use crate::specific::user_actions::{LoginData, UserAction};

pub fn main() {
	let login_action = UserAction::LOGIN(Ok(LoginData {
		name: "User 01".to_owned(),
		pass: "p4$$w0rd".to_owned(),
	}));
	let result = login_action.exec();
	println!("result: {:?}", result);
}
