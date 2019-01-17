use crate::schema::user;

use diesel::prelude::*;
use diesel::deserialize;

type ConstantIntegerFromSql=deserialize::FromSql<diesel::sql_types::Integer,diesel::mysql::Mysql>;

#[derive(Identifiable,Debug,Queryable,QueryableByName)]
#[has_many(content)]
pub struct User{
	pub id: i32,
	pub username: String,
	pub email: String,
	pub username_lower: String,
	pub email_lower: String,
	pub password_hash: String,
	pub name: String,
	pub avatar_url: Option<String>,
	pub permission: u32,
	pub created_at: chrono::NaiveDateTime,
	pub modified_at: chrono::NaiveDateTime,
	pub last_login_time: chrono::NaiveDateTime,
	pub status: UserStatus
}

#[derive(Copy,Clone,Serialize,Deserialize,Debug,FromSqlRow)]
#[repr(u8)]
#[serde(rename_all="lowercase")]
pub enum UserStatus{
	Normal=0,
	Deleted=1
}
impl ConstantIntegerFromSql for UserStatus{
	fn from_sql(bytes: Option<&[u8]>)->deserialize::Result<Self>{
		match <i32 as ConstantIntegerFromSql>::from_sql(bytes)?{
			0=>Ok(UserStatus::Normal),
			1=>Ok(UserStatus::Deleted),
			n=>Err(format!("Unknown UserStatus: {}",n))
		}
	}
}
