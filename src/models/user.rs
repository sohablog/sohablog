use diesel::prelude::*;
use serde_derive::*;
use chrono::{DateTime, Local, Utc};

use super::{Error, IntoInterface, RepositoryWrapper, Result};
use crate::{db::Database, schema::*, utils::*};

use bcrypt;

#[allow(dead_code)]
pub const PERM_LOGIN: i32 = 1 << 0; // login only
#[allow(dead_code)]
pub const PERM_POST_VIEW: i32 = 1 << 1; // view all posts (such as hidden post)
#[allow(dead_code)]
pub const PERM_POST_EDIT: i32 = 1 << 2; // create & edit post
#[allow(dead_code)]
pub const PERM_POST_DELETE: i32 = 1 << 3; // delete post
#[allow(dead_code)]
pub const PERM_CATEGORY_MANAGE: i32 = 1 << 4; // manage category
#[allow(dead_code)]
pub const PERM_COMMENT_MANAGE: i32 = 1 << 5; // manage category

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
	pub website: Option<String>,
	pub avatar_url: Option<String>,
	pub permission: i32,
	pub created_at: DateTime<Utc>,
	pub modified_at: DateTime<Utc>,
	pub last_login_time: DateTime<Utc>,
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
			.execute(&db.conn()?)
			.map(|_| ())
			.map_err(Error::from)
	}

	pub fn verify_password_hash(&self, pwd: &str) -> bool {
		bcrypt::verify(pwd, self.password_hash.as_ref()).unwrap_or(false)
	}

	pub fn has_permission(&self, perm: i32) -> bool {
		(self.permission & perm) != 0
	}

	pub fn check_permission(&self, perm: i32) -> Result<()> {
		match self.has_permission(perm) {
			true => Ok(()),
			false => Err(Error::UserHasNoPermission),
		}
	}

	pub fn to_session_info(&self) -> UserSessionInfo {
		UserSessionInfo {
			id: self.id,
			password_hash: self.password_hash.to_owned(),
		}
	}
}

use crate::interfaces::models::User as UserInterface;
impl UserInterface for RepositoryWrapper<User, Box<Database>> {
	fn id(&self) -> i32 {
		self.0.id
	}
	fn username(&self) -> &String {
		&self.0.username
	}
	fn name(&self) -> &String {
		&self.0.name
	}
	fn email(&self) -> &String {
		&self.0.email
	}
	fn website(&self) -> Option<&String> {
		self.0.website.as_ref()
	}
	fn avatar_url(&self) -> Option<&String> {
		self.0.avatar_url.as_ref()
	}
	fn permission(&self) -> i32 {
		self.0.permission
	}
	fn created_at(&self) -> &DateTime<Local> {
		&self.0.created_at.into()
	}
	fn modified_at(&self) -> &DateTime<Local> {
		&self.0.modified_at.into()
	}
	fn last_login_time(&self) -> &DateTime<Local> {
		&self.0.last_login_time.into()
	}
	fn status(&self) -> UserStatus {
		self.0.status
	}
}

impl IntoInterface<Box<dyn UserInterface>> for User {
	fn into_interface(self, db: &Box<Database>) -> Box<dyn UserInterface> {
		Box::new(RepositoryWrapper(self, db.clone())) as Box<dyn UserInterface>
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
	pub permission: i32,
}

use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome};
impl<'a, 'r> FromRequest<'a, 'r> for User {
	type Error = ();
	fn from_request(request: &'a rocket::request::Request<'r>) -> Outcome<User, ()> {
		let db = request.guard::<rocket::State<Box<Database>>>()?;
		let session: SessionInfo = request.guard::<SessionInfo>()?;
		session
			.user
			.as_ref()
			.and_then(|session| {
				User::find(&db, session.id).ok().and_then(|u| {
					if u.password_hash == session.password_hash {
						Some(u)
					} else {
						None
					}
				})
			})
			.or_forward(())
	}
}

pub use crate::types::UserStatus;
