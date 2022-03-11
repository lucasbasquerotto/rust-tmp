mod action;
mod specific;

use crate::action::{ActionInput, ActionRequest};
use crate::specific::actions::user::auth_actions::{Login, LoginData, LoginResult};

pub fn main() {
	let login = Login();
	let input = &ActionInput {
		request: LoginData {
			name: "User 01".to_owned(),
			pass: "p4$$w0rd".to_owned(),
		},
	};
	let result = login.run(&|| input);
	println!("result: {:?}", result);

	assert_eq!(
		result.unwrap(),
		LoginResult {
			id: 1,
			name: "User 01".to_string(),
		},
	);
}
