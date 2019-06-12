use crate::{
	db::Database,
	models::user,
};
use rocket::{
	http::Status,
	request::{State, FromRequest, Outcome, Request}
};
use std::str::FromStr;

use crate::routes::error::Error; // temp solution

#[derive(Debug)]
pub struct SystemConfig {
	pub upload_dir: String,
	pub upload_route: String,
	pub real_ip_header: Option<String>,
	pub is_prod: bool,
}

/// `GlobalContext` is a struct contained some globally useful items, such as user and database connection.
pub struct GlobalContext<'a> {
	pub ip: VisitorIP,
	pub db: State<'a, Database>,
	pub user: Option<user::User>,
	pub system_config: State<'a, SystemConfig>,
	pub user_agent: Option<String>,
}
impl<'a, 'r> FromRequest<'a, 'r> for GlobalContext<'r> {
	type Error = ();
	fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
		Outcome::Success(Self {
			ip: request.guard::<VisitorIP>().unwrap(), // FIXME: Needs to process errors properly
			db: request.guard::<State<Database>>()?,
			user: request.guard::<Option<user::User>>().unwrap(),
			system_config: request.guard::<State<SystemConfig>>()?,
			user_agent: request.headers().get_one("User-Agent").and_then(|s| Some(s.to_string()))
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
		let system_config = request.guard::<State<SystemConfig>>().unwrap();
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
