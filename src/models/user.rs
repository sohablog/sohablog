use diesel::prelude::*;
use serde_derive::*;

use super::{Error, Result};
use crate::{db::Database, schema::*};

use bcrypt;

#[allow(dead_code)]
pub const PERM_LOGIN: u32 = 1 << 0; // login only
#[allow(dead_code)]
pub const PERM_POST_VIEW: u32 = 1 << 1; // view all posts (such as hidden post)
#[allow(dead_code)]
pub const PERM_POST_EDIT: u32 = 1 << 2; // create & edit post
#[allow(dead_code)]
pub const PERM_POST_DELETE: u32 = 1 << 3; // delete post
#[allow(dead_code)]
pub const PERM_CATEGORY_MANAGE: u32 = 1 << 4; // manage category

#[derive(Identifiable, Debug, Queryable, Clone, Serialize)]
#[primary_key(id)]
#[table_name = "user"]
pub struct User {
	pub id: i32,
	pub username: String,
	pub password_hash: String,
	pub name: String,
	pub email: String,
	pub username_lower: String,
	pub email_lower: String,
	pub avatar_url: Option<String>,
	pub permission: u32,
	pub created_at: chrono::NaiveDateTime,
	pub modified_at: chrono::NaiveDateTime,
	pub last_login_time: chrono::NaiveDateTime,
	pub status: UserStatus,
}
impl User {
	last!(user);
	insert!(user, NewUser);
	find_pk!(user);
	find_one_by!(user, find_by_username, username as &str);

	pub fn generate_password_hash(pwd: &str) -> Result<String> {
		bcrypt::hash(pwd, 12).map_err(Error::from)
	}

	pub fn set_password_hash(&self, db: &Database, pwd: &str) -> Result<()> {
		diesel::update(self)
			.set(user::password_hash.eq(pwd))
			.execute(&*db.pool().get()?)
			.map(|_| ())
			.map_err(Error::from)
	}

	pub fn verify_password_hash(&self, pwd: &str) -> bool {
		bcrypt::verify(pwd, self.password_hash.as_ref()).unwrap_or(false)
	}

	pub fn has_permission(&self, perm: u32) -> bool {
		(self.permission & perm) != 0
	}

	pub fn check_permission(&self, perm: u32) -> Result<()> {
		match self.has_permission(perm) {
			true => Ok(()),
			false => Err(Error::UserHasNoPermission),
		}
	}
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser {
	pub username: String,
	pub email: String,
	pub username_lower: String,
	pub email_lower: String,
	pub password_hash: String,
	pub name: String,
	pub permission: u32,
}

use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome};
impl<'a, 'r> FromRequest<'a, 'r> for User {
	type Error = ();
	fn from_request(request: &'a rocket::request::Request<'r>) -> Outcome<User, ()> {
		let db = request.guard::<rocket::State<Database>>()?;
		request
			.cookies()
			.get_private("user_id")
			.and_then(|cookie| cookie.value().parse().ok())
			.and_then(|id| User::find(&db, id).ok())
			.or_forward(())
	}
}

// integer constants

use diesel::{
	deserialize::{self, FromSql},
	mysql::Mysql,
	sql_types::Integer,
};

#[derive(Copy, Clone, Serialize, Deserialize, Debug, FromSqlRow)]
#[repr(u8)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
	Normal = 0,
	Deleted = 1,
}
impl FromSql<Integer, Mysql> for UserStatus {
	fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
		match <i32 as FromSql<Integer, Mysql>>::from_sql(bytes)? {
			0 => Ok(UserStatus::Normal),
			1 => Ok(UserStatus::Deleted),
			n => Err(format!("Unknown UserStatus: {}", n).into()),
		}
	}
}
