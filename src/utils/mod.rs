mod page;
mod session_info;
mod csrf;
mod db;

pub use page::Page;
pub use session_info::{SessionInfo, UserSessionInfo};
pub use csrf::CSRFToken;
pub use db::DatabaseConnection;

use crate::interfaces::models::User;

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

#[derive(Debug)]
pub struct VisitorIP(pub std::net::IpAddr);
impl ToString for VisitorIP {
	fn to_string(&self) -> String {
		self.0.to_string()
	}
}

pub struct TemplateContext<'a> {
	pub ip: &'a VisitorIP,
	pub user: Option<Box<User>>,
	pub system_config: &'a SystemConfig,
	pub user_agent: Option<&'a String>,
	pub session_info: &'a SessionInfo,
}
