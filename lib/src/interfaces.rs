pub mod models {

	use crate::types::*;
	use chrono::{DateTime, Local};

	pub trait User {
		fn id(&self) -> i32;
		fn username(&self) -> &String;
		fn name(&self) -> &String;
		fn email(&self) -> &String;
		fn website(&self) -> Option<&String>;
		fn avatar_url(&self) -> Option<&String>;
		fn permission(&self) -> i32;
		fn created_at(&self) -> &DateTime<Local>;
		fn modified_at(&self) -> &DateTime<Local>;
		fn last_login_time(&self) -> &DateTime<Local>;
		fn status(&self) -> UserStatus;
	}

	pub trait Tag {
		fn name(&self) -> &String;
	}

	pub trait Category {
		fn id(&self) -> i32;
		fn slug(&self) -> &String;
		fn name(&self) -> &String;
		fn description(&self) -> Option<&String>;
		fn order(&self) -> i32;
		fn parent_id(&self) -> Option<i32>;
		fn parent(&self) -> Option<Box<dyn Category>>;
	}

	pub trait Content {
		fn id(&self) -> i32;
		fn user(&self) -> Box<dyn User>;
		fn created_at(&self) -> &DateTime<Local>;
		fn modified_at(&self) -> &DateTime<Local>;
		fn time(&self) -> &DateTime<Local>;
		fn title(&self) -> Option<&String>;
		fn slug(&self) -> Option<&String>;
		fn content(&self) -> &String;
		fn draft_content(&self) -> Option<&String>;
		fn order_level(&self) -> i32;
		fn r#type(&self) -> ContentType;
		fn status(&self) -> ContentStatus;
		fn allow_comment(&self) -> bool;
		fn category_id(&self) -> Option<i32>;
		fn category(&self) -> Option<Box<dyn Category>>;
		fn tags(&self) -> Vec<Box<dyn Tag>>;

		fn link(&self) -> String;
		fn get_tags_name(&self) -> Vec<String>;
		fn get_neighbor_post(&self, prev: bool) -> Option<Box<dyn Content>>;
		fn get_comment_url(&self) -> String;
		fn get_parent_comments(&self) -> Vec<Box<dyn Comment>>;
	}

	pub trait Author {
		fn name(&self) -> &String;
		fn mail(&self) -> Option<&String>;
		fn link(&self) -> Option<&String>;
		fn avatar_url(&self, default_url: &str) -> String;
	}

	pub trait Comment {
		fn id(&self) -> i32;
		fn author(&self) -> Box<dyn Author>;
		fn ip(&self) -> Option<&String>;
		fn user_agent(&self) -> Option<&String>;
		fn text(&self) -> &String;
		fn time(&self) -> &DateTime<Local>;
		fn status(&self) -> CommentStatus;
		fn reply_to(&self) -> Option<i32>;
		fn parent(&self) -> Option<Box<dyn Comment>>;
		fn content(&self) -> Box<dyn Content>;

		fn children(&self) -> Vec<Box<dyn Comment>>;
	}

}
