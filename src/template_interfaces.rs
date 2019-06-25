pub mod models {

	use chrono::NaiveDateTime;
	use crate::types::*;

	pub trait User {
		fn id(&self) -> i32;
		fn username(&self) -> String;
		fn password_hash(&self) -> String;
		fn name(&self) -> String;
		fn email(&self) -> String;
		fn username_lower(&self) -> String;
		fn email_lower(&self) -> String;
		fn website(&self) -> Option<String>;
		fn avatar_url(&self) -> Option<String>;
		fn permission(&self) -> u32;
		fn created_at(&self) -> NaiveDateTime;
		fn modified_at(&self) -> NaiveDateTime;
		fn last_login_time(&self) -> NaiveDateTime;
		fn status(&self) -> UserStatus;
	}

	pub trait Tag {
		fn name(&self) -> String;
	}

	pub trait Category {
		fn slug(&self) -> String;
		fn name(&self) -> String;
		fn description(&self) -> Option<String>;
		fn order(&self) -> i32;
		fn parent(&self) -> Option<i32>;
	}

	pub trait Content {
		fn id(&self) -> i32;
		fn user(&self) -> Box<User>;
		fn created_at(&self) -> NaiveDateTime;
		fn modified_at(&self) -> NaiveDateTime;
		fn time(&self) -> NaiveDateTime;
		fn title(&self) -> Option<String>;
		fn slug(&self) -> Option<String>;
		fn content(&self) -> String;
		fn draft_content(&self) -> Option<String>;
		fn order_level(&self) -> i32;
		fn r#type(&self) -> ContentType;
		fn status(&self) -> ContentStatus;
		fn allow_comment(&self) -> bool;
		fn category(&self) -> Option<Box<Category>>;

		fn link(&self) -> String;
		fn tags(&self) -> Vec<Box<Tag>>;
		fn get_tags_name(&self) -> Vec<String>;
		fn get_neighbor_post(&self, prev: bool) -> Box<Content>;
		fn get_comment_url(&self) -> String;
		fn get_parent_comments(&self) -> Vec<Box<Comment>>;
	}

	pub trait Author {
		fn name(&self) -> String;
		fn mail(&self) -> Option<String>;
		fn link(&self) -> Option<String>;
	}

	pub trait Comment {
		fn id(&self) -> i32;
		fn author(&self) -> Box<Author>;
		fn ip(&self) -> Option<String>;
		fn user_agent(&self) -> Option<String>;
		fn text(&self) -> String;
		fn time(&self) -> NaiveDateTime;
		fn status(&self) -> CommentStatus;
		fn reply_to(&self) -> Option<i32>;
		fn parent(&self) -> Option<Box<Comment>>;
		fn content(&self) -> Box<Content>;

		fn children(&self) -> Vec<Box<Comment>>;
	}

}
