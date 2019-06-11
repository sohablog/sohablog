use diesel::prelude::*;
use serde_derive::*;

use super::{
	content::Content,
	user::User,
	Error, Result,
};
use crate::{db::Database, schema::*, templates::ToHtml};

#[derive(Debug, Queryable, Associations, Clone, Serialize, Identifiable, AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "comment"]
#[primary_key(id)]
#[belongs_to(User, foreign_key = "user")]
#[belongs_to(Content, foreign_key = "content")]
#[belongs_to(Comment, foreign_key = "parent")]
// #[belongs_to(Comment, foreign_key = "reply_to")] <- diesel doesn't support foreign keys more than one;
pub struct Comment {
	pub id: i32,
	pub user: Option<i32>,
	pub author_name: String,
	pub author_mail: Option<String>,
	pub author_link: Option<String>,
	pub ip: Option<String>,
	pub user_agent: Option<String>,
	pub text: String,
	pub time: chrono::NaiveDateTime,
	pub status: CommentStatus,
	pub reply_to: Option<i32>,
	pub parent: Option<i32>,
	pub content: i32,
}
#[derive(Insertable, Debug)]
#[table_name = "comment"]
pub struct NewComment {
	pub user: Option<i32>,
	pub author_name: String,
	pub author_mail: Option<String>,
	pub author_link: Option<String>,
	pub ip: Option<String>,
	pub user_agent: Option<String>,
	pub text: String,
	pub status: CommentStatus,
	pub reply_to: Option<i32>,
	pub parent: Option<i32>,
	pub content: i32,
}
impl Comment {
	last!(comment);
	insert!(comment, NewComment);
	find_pk!(comment);
	update!();

	pub fn new(author: Author, ip: Option<String>, ua: Option<String>, text: String, reply_to: Option<i32>, parent: Option<i32>, content_id: i32, status: CommentStatus) -> NewComment {
		NewComment {
			user: author.local_user,
			author_name: author.name.to_owned(),
			author_mail: author.mail.to_owned(),
			author_link: author.link.to_owned(),
			ip: ip,
			user_agent: ua,
			text: text,
			status: status,
			reply_to: reply_to,
			parent: parent,
			content: content_id
		}
	}
}

#[derive(Debug)]
pub struct Author {
	pub name: String,
	pub mail: Option<String>,
	pub link: Option<String>,
	pub local_user: Option<i32>
}
impl Author {
	pub fn from_user(user: &User) -> Self {
		Self {
			name: user.name.to_owned(),
			mail: Some(user.email.to_owned()),
			link: user.website.to_owned(),
			local_user: Some(user.id)
		}
	}

	pub fn new(name: String, mail: Option<String>, link: Option<String>) -> Self {
		Self {
			name: name,
			mail: mail,
			link: link,
			local_user: None
		}
	}
}

//integer constants
use diesel::{
	deserialize::{self, FromSql},
	mysql::Mysql,
	serialize::{self, ToSql},
	sql_types::Integer,
};

#[derive(Copy, Clone, Serialize, Deserialize, Debug, FromSqlRow, AsExpression, PartialEq)]
#[repr(u8)]
#[sql_type = "Integer"]
#[serde(rename_all = "lowercase")]
pub enum CommentStatus {
	Normal = 0,
	Deleted = 1,
	Spam = 2,
	PendingReview = 3,
}
impl CommentStatus {
	// not impl std::convert::TryFrom　for some reasons
	pub fn try_from(n: i32) -> Result<Self> {
		match n {
			0 => Ok(CommentStatus::Normal),
			1 => Ok(CommentStatus::Deleted),
			2 => Ok(CommentStatus::Spam),
			3 => Ok(CommentStatus::PendingReview),
			n => Err(Error::NoEnumNumber("CommentStatus".to_string(), n)),
		}
	}
}
impl FromSql<Integer, Mysql> for CommentStatus {
	fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
		let i = <i32 as FromSql<Integer, Mysql>>::from_sql(bytes)?;
		match Self::try_from(i) {
			Ok(s) => Ok(s),
			Err(_) => Err(format!("Failed convert `{}` to CommentStatus", i).into()),
		}
	}
}
impl ToSql<Integer, Mysql> for CommentStatus {
	fn to_sql<W: std::io::Write>(
		&self,
		out: &mut serialize::Output<W, Mysql>,
	) -> serialize::Result {
		ToSql::<Integer, Mysql>::to_sql(&(*self as i32), out)
	}
}
impl ToHtml for CommentStatus {
	fn to_html(&self, out: &mut dyn std::io::Write) -> std::io::Result<()> {
		write!(out, "{}", *self as i32)
	}
}