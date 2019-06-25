use crate::{db::Database, models::{user, IntoInterface}};
pub use crate::utils::*;
use rocket::{
	fairing::{Fairing, Info as FairingInfo, Kind as FairingKind},
	http::{Cookie, Cookies, Method, Status},
	request::{FromRequest, Outcome, Request, State},
	Data,
};
use std::str::FromStr;

use crate::routes::error::Error; // temp solution

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

/// `GlobalContext` is a struct contained some globally useful items, such as user and database connection.
pub struct GlobalContext<'a> {
	pub ip: VisitorIP,
	pub db: Box<Database>,
	pub user: Option<user::User>,
	pub system_config: &'a SystemConfig,
	pub user_agent: Option<String>,
	pub session_info: SessionInfo,
}
impl<'a> GlobalContext<'a> {
	pub fn get_template_context(&self) -> TemplateContext {
		TemplateContext {
			ip: &self.ip,
			user: self.user.clone().into_interface(&self.db),
			system_config: &self.system_config,
			user_agent: self.user_agent.as_ref(),
			session_info: &self.session_info,
		}
	}
}
impl<'a, 'r> FromRequest<'a, 'r> for GlobalContext<'r> {
	type Error = ();
	fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
		Outcome::Success(Self {
			ip: request.guard::<VisitorIP>().unwrap(), // FIXME: Needs to process errors properly
			db: request.guard::<State<Box<Database>>>()?.inner().clone(),
			user: request.guard::<Option<user::User>>().unwrap(),
			system_config: request.guard::<State<SystemConfig>>()?.inner(),
			user_agent: request
				.headers()
				.get_one("User-Agent")
				.and_then(|s| Some(s.to_string())),
			session_info: request.guard::<SessionInfo>()?,
		})
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for VisitorIP {
	type Error = Error;
	fn from_request(request: &'a Request<'r>) -> Outcome<Self, Error> {
		let system_config = request.guard::<'a, State<SystemConfig>>().unwrap();
		let remote = request.remote().and_then(|o| Some(o.ip()));
		let real_ip = system_config
			.real_ip_header
			.as_ref()
			.and_then(|o| request.headers().get_one(o.as_str()));
		let ip_addr = if let Some(ip_str) = real_ip {
			std::net::IpAddr::from_str(ip_str).ok()
		} else {
			remote
		};
		match ip_addr {
			Some(ip) => Outcome::Success(Self(ip)),
			None => Outcome::Failure((Status::BadRequest, Error::BadRequest("Invalid remote IP"))),
		}
	}
}

#[derive(Debug)]
pub struct CSRFTokenValidation(pub Option<String>);
impl Fairing for CSRFTokenValidation {
	fn info(&self) -> FairingInfo {
		FairingInfo {
			name: "CSRF Validation Finder",
			kind: FairingKind::Request,
		}
	}

	// FIXME: or as a feature?
	/// `csrf_field` should appear in the front of the form, cuz we didn't use data.peek for the second time. The full stream will not be loaded.
	fn on_request(&self, request: &mut Request, data: &Data) {
		let system_config = request.guard::<State<SystemConfig>>().unwrap();

		if request.method() == Method::Post
			|| request.method() == Method::Put
			|| request.method() == Method::Delete
		{
			let token = if request
				.content_type()
				.map(|c| c.media_type())
				.filter(|m| m.top() == "multipart" && m.sub() == "form-data")
				.is_some()
			{
				let field_disposition_str: String = format!(
					"Content-Disposition: form-data; name=\"{}\"",
					&system_config.csrf_field_name
				);
				let field_disposition = field_disposition_str.as_bytes();
				data.peek()
					.split(|&c| c == 0x0a || c == 0x0d)
					.filter(|d| !d.is_empty())
					.skip_while(|&d| {
						d != field_disposition
							&& d != &field_disposition[..field_disposition.len() - 2]
					})
					.skip(1)
					.map(|s| s.split(|&c| c == 0x0a || c == 0x0d).next())
					.next()
					.unwrap_or(None)
					.and_then(|b| std::str::from_utf8(b).ok())
			} else {
				std::str::from_utf8(data.peek())
					.unwrap_or("")
					.split('&')
					.filter_map(|s| {
						s.find('=').and_then(|l| {
							let (key, value) = s.split_at(l + 1);
							let key = &key[0..l];
							if key == system_config.csrf_field_name.as_str() {
								Some(value)
							} else {
								None
							}
						})
					})
					.next()
			}
			.and_then(|s| Some(String::from(s)));
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
