use crate::{
	db::Database,
	models::user,
};
use rocket::{
	fairing::{Fairing, Kind as FairingKind, Info as FairingInfo},
	http::{Status, Cookies, Cookie, Method},
	request::{State, FromRequest, Outcome, Request},
	Data,
};
use uuid::Uuid;
use serde_derive::*;
use std::{str::FromStr, default::Default};

use crate::routes::error::Error; // temp solution

#[derive(Debug)]
pub struct SystemConfig {
	pub upload_dir: String,
	pub upload_route: String,
	pub session_name: String,
	pub real_ip_header: Option<String>,
	pub csrf_cookie_name: Option<String>,
	pub csrf_field_name: String,
	pub is_prod: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionInfo {
	pub user: Option<user::UserSessionInfo>,
	pub csrf_token: CSRFToken,
}
impl SessionInfo {
	pub fn persist(&self, cookies: &mut Cookies, system_config: &SystemConfig) {
		cookies.add_private(
			Cookie::build(system_config.session_name.to_owned(), serde_json::to_string(self).unwrap_or("".into()))
				.path("/")
				.finish()
		)
	}
}
impl Default for SessionInfo {
	fn default() -> Self {
		Self {
			user: None,
			csrf_token: Uuid::new_v4().into()
		}
	}
}
impl<'a, 'r> FromRequest<'a, 'r> for SessionInfo {
	type Error = ();
	fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
		let system_config = request.guard::<State<SystemConfig>>().unwrap();
		let mut cookies = request.cookies();
		Outcome::Success(cookies
			.get_private(&system_config.session_name.as_str())
			.and_then(|c| serde_json::from_str::<SessionInfo>(c.value()).ok())
			.unwrap_or_default()
		)
	}
}

/// `GlobalContext` is a struct contained some globally useful items, such as user and database connection.
pub struct GlobalContext<'a> {
	pub ip: VisitorIP,
	pub db: State<'a, Database>,
	pub user: Option<user::User>,
	pub system_config: State<'a, SystemConfig>,
	pub user_agent: Option<String>,
	pub session_info: SessionInfo,
}
impl<'a, 'r> FromRequest<'a, 'r> for GlobalContext<'r> {
	type Error = ();
	fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
		Outcome::Success(Self {
			ip: request.guard::<VisitorIP>().unwrap(), // FIXME: Needs to process errors properly
			db: request.guard::<State<Database>>()?,
			user: request.guard::<Option<user::User>>().unwrap(),
			system_config: request.guard::<State<SystemConfig>>()?,
			user_agent: request.headers().get_one("User-Agent").and_then(|s| Some(s.to_string())),
			session_info: request.guard::<SessionInfo>()?
		})
	}
}

#[derive(Debug)]
pub struct VisitorIP(std::net::IpAddr);
impl ToString for VisitorIP {
	fn to_string(&self) -> String {
		self.0.to_string()
	}
}
impl<'a, 'r> FromRequest<'a, 'r> for VisitorIP {
	type Error = Error;
	fn from_request(request: &'a Request<'r>) -> Outcome<Self, Error> {
		let system_config = request.guard::<'a, State<SystemConfig>>().unwrap();
		let remote = request.remote().and_then(|o| Some(o.ip()));
		let real_ip = system_config.real_ip_header.as_ref().and_then(|o| request.headers().get_one(o.as_str()));
		let ip_addr = if let Some(ip_str) = real_ip {
			std::net::IpAddr::from_str(ip_str).ok()
		} else {
			remote
		};
		match ip_addr {
			Some(ip) => Outcome::Success(Self(ip)),
			None => Outcome::Failure((Status::BadRequest, Error::BadRequest("Invalid remote IP")))
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CSRFToken(String);
impl CSRFToken {
	pub fn validate(&self, s: &String) -> Result<(), Error> {
		if &self.0 == s {
			Ok(())
		} else {
			Err(Error::CSRFViolation)
		}
	}
}
impl From<Uuid> for CSRFToken {
	fn from(uuid: Uuid) -> Self {
		Self(uuid.to_simple().to_string())
	}
}
impl From<String> for CSRFToken {
	fn from(s: String) -> Self {
		Self(s)
	}
}

pub struct CSRFTokenValidation(pub Option<String>);
impl Fairing for CSRFTokenValidation {
	fn info(&self) -> FairingInfo {
		FairingInfo {
			name: "CSRF Validation Finder",
			kind: FairingKind::Request
		}
	}
	
	fn on_request(&self, request: &mut Request, data: &Data) {
		let system_config = request.guard::<State<SystemConfig>>().unwrap();

		if request.method() == Method::Post || request.method() == Method::Put || request.method() == Method::Delete {
			let token = if request.content_type().map(|c| c.media_type()).filter(|m| m.top() == "multipart" && m.sub() == "form-data").is_some() {
				let field_disposition_str: String = format!("Content-Disposition: form-data; name=\"{}\"", &system_config.csrf_field_name);
				let field_disposition = field_disposition_str.as_bytes();
				data.peek()
					.split(|&c| c==0x0a || c==0x0d)
					.filter(|d| !d.is_empty())
					.skip_while(|&d| d != field_disposition && d != &field_disposition[..field_disposition.len() - 2])
					.skip(1)
					.map(|s| s.split(|&c| c==0x0a || c==0x0d).next())
					.next()
					.unwrap_or(None)
					.and_then(|b| std::str::from_utf8(b).ok())
			} else {
				std::str::from_utf8(data.peek()).unwrap_or("")
					.split('&')
					.filter_map(|s| s.find('=').and_then(|l| {
						let (key, value) = s.split_at(l + 1);
						if key == system_config.csrf_field_name.as_str() {
							Some(value)
						} else {
							None
						}
					}))
					.next()
			}.and_then(|s| Some(String::from(s)));
			request.local_cache(|| CSRFTokenValidation(token));
		}
	}
}
impl<'a, 'r> FromRequest<'a, 'r> for CSRFTokenValidation {
	type Error = Error;
	fn from_request(request: &'a Request<'r>) -> Outcome<Self, Error> {
		let token: &CSRFTokenValidation = request.local_cache(|| CSRFTokenValidation(None));
		if let Some(token) = &token.0 {
			if let Outcome::Success(session_info) = request.guard::<SessionInfo>() {
				if let Ok(_) = session_info.csrf_token.validate(&token) {
					return Outcome::Success(Self(None));
				}
			}
		}
		Outcome::Failure((Status::BadRequest, Error::CSRFViolation))
	}
}
