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

mod static_file;
pub use static_file::StaticFile;

use crate::interfaces::models::User;
use crate::render::RenderHelper;

#[derive(Debug)]
pub struct SystemConfig {
	pub plugin_dir: String,
	pub upload_dir: String,
	pub upload_route: String,
	pub session_name: String,
	pub robots_txt_path: String,
	pub real_ip_header: Option<String>,
	pub csrf_cookie_name: Option<String>,
	pub csrf_field_name: String,
	pub is_prod: bool,
	pub theme_name: String,
}

pub struct TemplateContext<'a> {
	pub ip: &'a VisitorIP,
	pub user: Option<Box<dyn User>>,
	pub system_config: &'a SystemConfig,
	pub user_agent: Option<&'a String>,
	pub session_info: &'a SessionInfo,
	pub render_helper: Box<dyn RenderHelper>,
}
