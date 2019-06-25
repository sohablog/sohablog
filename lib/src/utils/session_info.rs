#[cfg(feature = "main")]
use serde_derive::*;
#[cfg(feature = "main")]
use rocket::{
	http::{Cookie, Cookies},
	request::{FromRequest, Outcome, Request, State},
};
#[cfg(feature = "main")]
use crate::utils::SystemConfig;

use uuid::Uuid;
use super::CSRFToken;

#[derive(Debug)]
#[cfg_attr(feature = "main", derive(Serialize, Deserialize))]
pub struct SessionInfo {
	pub user: Option<UserSessionInfo>,
	pub csrf_token: CSRFToken,
}
#[cfg(feature = "main")]
impl SessionInfo {
	pub fn persist(&self, cookies: &mut Cookies, system_config: &SystemConfig) {
		if let Some(cookie_name) = &system_config.csrf_cookie_name {
			cookies.add(
				Cookie::build(cookie_name.to_owned(), self.csrf_token.to_string())
					.max_age(time::Duration::days(3))
					.path("/")
					.finish(),
			);
		}

		cookies.add_private(
			Cookie::build(
				system_config.session_name.to_owned(),
				serde_json::to_string(self).unwrap_or("".into()),
			)
			.path("/")
			.finish(),
		);
	}
}
impl Default for SessionInfo {
	fn default() -> Self {
		Self {
			user: None,
			csrf_token: Uuid::new_v4().into(),
		}
	}
}
#[cfg(feature = "main")]
impl<'a, 'r> FromRequest<'a, 'r> for SessionInfo {
	type Error = ();
	fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
		let system_config = request.guard::<State<SystemConfig>>().unwrap();
		let mut cookies = request.cookies();
		let session = cookies
			.get_private(&system_config.session_name.as_str())
			.and_then(|c| serde_json::from_str::<SessionInfo>(c.value()).ok())
			.unwrap_or_default();
		session.persist(&mut cookies, &system_config);
		Outcome::Success(session)
	}
}

#[derive(Debug)]
#[cfg_attr(feature = "main", derive(Serialize, Deserialize))]
pub struct UserSessionInfo {
	pub id: i32,
	pub password_hash: String,
}
