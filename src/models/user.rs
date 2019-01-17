use diesel::prelude::*;
use serde_derive::*;

use crate::schema::*;
use super::{
	Error,
	Result
};

use bcrypt;

#[derive(Identifiable,Debug,Queryable,Clone)]
#[primary_key(id)]
#[table_name="user"]
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
impl User{
	pub fn generate_password_hash(pwd: &str)->Result<String>{
		bcrypt::hash(pwd,12).map_err(Error::from)
	}
}

// integer constants

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
pub enum UserStatus{
	Normal=0,
	Deleted=1
}
impl FromSql<Integer,Mysql> for UserStatus{
	fn from_sql(bytes: Option<&[u8]>)->deserialize::Result<Self>{
		match <i32 as FromSql<Integer,Mysql>>::from_sql(bytes)?{
			0=>Ok(UserStatus::Normal),
			1=>Ok(UserStatus::Deleted),
			n=>Err(format!("Unknown UserStatus: {}",n).into())
		}
	}
}
