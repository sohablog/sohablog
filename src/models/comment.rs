use diesel::prelude::*;
use serde_derive::*;

use super::{content::Content, user::User, Error, Result, RepositoryWrapper};
use crate::{db::Database, utils::*, schema::*, templates::ToHtml};
use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Debug, Queryable, Associations, Clone, Identifiable, AsChangeset)]
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
	pub time: NaiveDateTime,
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
#[derive(Serialize)]
pub struct CommentSerializedNormal {
	pub id: i32,
	pub name: String,
	pub mail: Option<String>,
	pub link: Option<String>,
	pub text: String,
	pub time: DateTime<Utc>,
	pub reply_to: Option<i32>,
}
impl Comment {
	last!(comment);
	insert!(comment, NewComment);
	find_pk!(comment);
	find_by!(comment, find_by_content_id, content as i32);
	find_by!(comment, find_by_parent, parent as i32, status as CommentStatus);
	update!();

	pub fn count_by_status(db: &Database, status: &CommentStatus) -> Result<i64> {
		comment::table
			.filter(comment::status.eq(status))
			.count()
			.get_result(&db.conn()?)
			.map_err(Error::from)
	}

	pub fn find_by_status(
		db: &Database,
		(min, max): (i32, i32),
		status: &CommentStatus
	) -> Result<Vec<Self>> {
		let mut query = comment::table.into_boxed();

		query = query
			.filter(comment::status.eq(status))
			.order(comment::time.desc())
			.offset(min.into()).limit((max - min).into());
		query.load::<Self>(&db.conn()?).map_err(Error::from)
	}

	pub fn get_children(&self, db: &Database) -> Result<Vec<Self>> {
		Self::find_by_parent(db, self.id, CommentStatus::Normal)
	}

	pub fn serialize_normal(&self) -> CommentSerializedNormal {
		CommentSerializedNormal {
			id: self.id,
			name: self.author_name.to_owned(),
			mail: self.author_mail.to_owned(),
			link: self.author_link.to_owned(),
			text: self.text.to_owned(),
			time: DateTime::<Utc>::from_utc(self.time.to_owned(), Utc),
			reply_to: self.reply_to,
		}
	}

	pub fn find_parents_by_content_id(db: &Database, content_id: i32) -> Result<Vec<Self>> {
		comment::table
			.filter(comment::status.eq(CommentStatus::Normal))
			.filter(comment::parent.is_null())
			.filter(comment::content.eq(content_id))
			.load::<Self>(&db.conn()?)
			.map_err(Error::from)
	}

	pub fn get_content(&self, db: &Database) -> Result<Content> {
		Content::find(db, self.content)
	}

	pub fn new(
		author: Author,
		ip: Option<String>,
		ua: Option<String>,
		text: String,
		reply_to: Option<i32>,
		parent: Option<i32>,
		content_id: i32,
		status: CommentStatus,
	) -> NewComment {
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
			content: content_id,
		}
	}
}

use crate::template_interfaces::models::{
	Comment as CommentInterface,
	Author as AuthorInterface,
	Content as ContentInterface,
};
impl CommentInterface for RepositoryWrapper<Comment, &'static Database> {
	fn id(&self) -> i32 { self.0.id }
	fn author(&self) -> Box<AuthorInterface> {
		Box::new(if let Some(user) = self.0.user.and_then(|uid| User::find(self.1, uid).ok()) {
			Author::from_user(&user)
		} else {
			Author {
				local_user: None,
				name: self.0.author_name.to_owned(),
				mail: self.0.author_mail.to_owned(),
				link: self.0.author_link.to_owned(),
			}
		}) as Box<AuthorInterface>
	}
	fn ip(&self) -> Option<&String> { self.0.ip.as_ref() }
	fn user_agent(&self) -> Option<&String> { self.0.user_agent.as_ref() }
	fn text(&self) -> &String { &self.0.text }
	fn time(&self) -> &chrono::NaiveDateTime { &self.0.time }
	fn status(&self) -> CommentStatus { self.0.status }
	fn reply_to(&self) -> Option<i32> { self.0.reply_to }

	fn parent(&self) -> Option<Box<CommentInterface>> {
		self.0.parent.map(|id| Box::new(RepositoryWrapper(Comment::find(self.1, id).unwrap(), self.1)) as Box<CommentInterface>)
	}
	fn content(&self) -> Box<ContentInterface> {
		Box::new(RepositoryWrapper(Content::find(self.1, self.0.content).unwrap(), self.1)) as Box<ContentInterface>
	}
	
	fn children(&self) -> Vec<Box<CommentInterface>> {
		self.0.get_children(self.1).unwrap().into_iter().map(|c| Box::new(RepositoryWrapper(c, self.1)) as Box<CommentInterface>).collect::<Vec<Box<CommentInterface>>>()
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
	pub name: String,
	pub mail: Option<String>,
	pub link: Option<String>,
	pub local_user: Option<i32>,
}
impl Author {
	pub fn from_user(user: &User) -> Self {
		Self {
			name: user.name.to_owned(),
			mail: Some(user.email.to_owned()),
			link: user.website.to_owned(),
			local_user: Some(user.id),
		}
	}

	pub fn new(name: String, mail: Option<String>, link: Option<String>) -> Self {
		Self {
			name: name,
			mail: mail,
			link: link,
			local_user: None,
		}
	}
}
impl AuthorInterface for Author {
	fn name(&self) -> &String { &self.name }
	fn mail(&self) -> Option<&String> { self.mail.as_ref() }
	fn link(&self) -> Option<&String> { self.link.as_ref() }
}

//integer constants
use diesel::{
	deserialize::{self, FromSql},
	mysql::Mysql,
	serialize::{self, ToSql},
	sql_types::Integer,
};

pub use crate::types::CommentStatus;
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
use rocket::{
	http::{
		uri::{self, FromUriParam, Query, UriDisplay},
		RawStr,
	},
	request::FromFormValue,
};
impl<'a> FromFormValue<'a> for CommentStatus {
	type Error = &'a RawStr;
	fn default() -> Option<Self> {
		Some(Self::Normal)
	}
	fn from_form_value(form_value: &'a RawStr) -> std::result::Result<Self, &'a RawStr> {
		match form_value.parse::<i32>() {
			Ok(status) => Ok(Self::try_from(status).map_err(|_| RawStr::from_str("No such CommentStatus"))?),
			_ => Err("Error when parsing `CommentStatus`".into()),
		}
	}
}
impl FromUriParam<Query, Option<CommentStatus>> for CommentStatus {
	type Target = CommentStatus;
	fn from_uri_param(v: Option<Self>) -> Self {
		v.unwrap_or(Self::Normal)
	}
}
impl UriDisplay<Query> for CommentStatus {
	fn fmt(&self, f: &mut uri::Formatter<Query>) -> std::fmt::Result {
		f.write_value(&format!("{}", *self as i32))
	}
}
