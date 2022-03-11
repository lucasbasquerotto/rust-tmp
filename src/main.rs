mod action;
mod specific;

use std::fmt::Debug;

use crate::action::{ActionInput, GeneralAction};
use crate::specific::user_actions::{LoginData, LoginResult, UserAction};

pub fn main() {
	let login_action = UserAction::LOGIN(ActionInput {
		request: LoginData {
			name: "User 01".to_owned(),
			pass: "p4$$w0rd".to_owned(),
		},
	});
	let result = login_action.full_run();
	println!("result: {:?}", result);

	assert!(same_val(
		&result,
		&Box::new(LoginResult {
			id: 1,
			name: "User 01".to_string(),
		}),
	));
}

fn same_val<T1, T2>(e1: &T1, e2: &T2) -> bool
where
	T1: Debug,
	T2: Debug,
{
	format!("{:?}", e1) == format!("{:?}", e2)
}
