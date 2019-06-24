mod page;
mod session_info;
mod csrf;

pub use page::Page;
pub use session_info::{SessionInfo, UserSessionInfo};
pub use csrf::CSRFToken;

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
