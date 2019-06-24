use uuid::Uuid;
use serde_derive::*;
use super::CSRFToken;

#[derive(Debug)]
#[cfg_attr(not(feature = "lib-only"), derive(Serialize, Deserialize))]
pub struct SessionInfo {
	pub user: Option<UserSessionInfo>,
	pub csrf_token: CSRFToken,
}
impl Default for SessionInfo {
	fn default() -> Self {
		Self {
			user: None,
			csrf_token: Uuid::new_v4().into(),
		}
	}
}

#[derive(Debug)]
#[cfg_attr(not(feature = "lib-only"), derive(Serialize, Deserialize))]
pub struct UserSessionInfo {
	pub id: i32,
	pub password_hash: String,
}
