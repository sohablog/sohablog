use crate::{db::Database, models::{user, IntoInterface}, plugin::PluginManager};
pub use crate::utils::*;
use rocket::{
	fairing::{Fairing, Info as FairingInfo, Kind as FairingKind},
	http::{Method, Status},
	request::{FromRequest, Outcome, Request, State},
	Data,
};

use crate::routes::error::Error; // temp solution

/// `GlobalContext` is a struct contained some globally useful items, such as user and database connection.
pub struct GlobalContext<'a> {
	pub ip: VisitorIP,
	pub db: Box<Database>,
	pub user: Option<user::User>,
	pub system_config: &'a SystemConfig,
	pub user_agent: Option<String>,
	pub session_info: SessionInfo,
	pub plugin_manager: State<'a, PluginManager>,
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
			plugin_manager: request.guard::<State<PluginManager>>()?,
		})
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
