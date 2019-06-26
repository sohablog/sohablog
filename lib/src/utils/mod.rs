mod page;
pub use page::Page;

mod session_info;
pub use session_info::{SessionInfo, UserSessionInfo};

mod csrf;
pub use csrf::CSRFToken;

mod visitor_ip;
pub use visitor_ip::VisitorIP;

mod db;
pub use db::DatabaseConnection;

use crate::interfaces::models::User;

#[derive(Debug)]
pub struct SystemConfig {
	pub plugin_dir: String,
	pub upload_dir: String,
	pub upload_route: String,
	pub session_name: String,
	pub real_ip_header: Option<String>,
	pub csrf_cookie_name: Option<String>,
	pub csrf_field_name: String,
	pub is_prod: bool,
	pub theme_name: String,
}

pub struct TemplateContext<'a> {
	pub ip: &'a VisitorIP,
	pub user: Option<Box<User>>,
	pub system_config: &'a SystemConfig,
	pub user_agent: Option<&'a String>,
	pub session_info: &'a SessionInfo,
}
