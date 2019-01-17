use diesel::prelude::*;
use serde_derive::*;

use crate::schema::*;
use super::{
	user::User
};

#[derive(Identifiable,Debug,Queryable,Associations)]
#[primary_key(id)]
#[table_name="content"]
#[belongs_to(User,foreign_key="user")]
pub struct Content{
	pub id: i32,
	pub user: i32,
	pub created_at: chrono::NaiveDateTime,
	pub modified_at: chrono::NaiveDateTime,
	pub title: Option<String>,
	pub content: String,
	pub order_level: i32,
	pub r#type: ContentType,
	pub status: ContentStatus,
	pub view_password: Option<String>,
	pub allow_comment: bool,
	pub allow_feed: bool,
	pub parent: Option<i32>
}

//integer constants

use diesel::{
	deserialize::{
		FromSql,
		self
	},
	sql_types::Integer,
	mysql::Mysql,
};

#[derive(Copy,Clone,Serialize,Deserialize,Debug,FromSqlRow)]
#[repr(u8)]
#[serde(rename_all="lowercase")]
pub enum ContentStatus{
	Normal=0,
	Deleted=1,
	Hidden=2
}
impl FromSql<Integer,Mysql> for ContentStatus{
	fn from_sql(bytes: Option<&[u8]>)->deserialize::Result<Self>{
		match <i32 as FromSql<Integer,Mysql>>::from_sql(bytes)?{
			0=>Ok(ContentStatus::Normal),
			1=>Ok(ContentStatus::Deleted),
			2=>Ok(ContentStatus::Hidden),
			n=>Err(format!("Unknown ContentStatus: {}",n).into())
		}
	}
}

#[derive(Copy,Clone,Serialize,Deserialize,Debug,FromSqlRow)]
#[repr(u8)]
#[serde(rename_all="lowercase")]
pub enum ContentType{
	Article=0,
	Draft=1
}
impl FromSql<Integer,Mysql> for ContentType{
	fn from_sql(bytes: Option<&[u8]>)->deserialize::Result<Self>{
		match <i32 as FromSql<Integer,Mysql>>::from_sql(bytes)?{
			0=>Ok(ContentType::Article),
			1=>Ok(ContentType::Draft),
			n=>Err(format!("Unknown ContentType: {}",n).into())
		}
	}
}
