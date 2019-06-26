use std::net::IpAddr;

#[derive(Debug)]
pub struct VisitorIP(pub IpAddr);
impl ToString for VisitorIP {
	fn to_string(&self) -> String {
		self.0.to_string()
	}
}

#[cfg(feature = "main")]
use crate::utils::SystemConfig;
#[cfg(feature = "main")]
use rocket::{
	http::Status,
	request::{FromRequest, Outcome, Request, State},
};
#[cfg(feature = "main")]
use std::str::FromStr;

#[cfg(feature = "main")]
impl<'a, 'r> FromRequest<'a, 'r> for VisitorIP {
	type Error = &'static str;
	fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
		let system_config = request.guard::<'a, State<SystemConfig>>().unwrap();
		let remote = request.remote().and_then(|o| Some(o.ip()));
		let real_ip = system_config
			.real_ip_header
			.as_ref()
			.and_then(|o| request.headers().get_one(o.as_str()));
		let ip_addr = if let Some(ip_str) = real_ip {
			IpAddr::from_str(ip_str).ok()
		} else {
			remote
		};
		match ip_addr {
			Some(ip) => Outcome::Success(Self(ip)),
			None => Outcome::Failure((Status::BadRequest, "Invalid remote IP")),
		}
	}
}
