use diesel::prelude::*;
use rocket_codegen::uri;
use serde_derive::*;

use super::{
	category::Category,
	tag::{AssocTagContent, Tag},
	user::User,
	Error, Result,
};
use crate::{db::Database, schema::*, templates::ToHtml};

#[derive(Debug, Queryable, Associations, Clone, Serialize, Identifiable, AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "content"]
#[primary_key(id)]
#[belongs_to(User, foreign_key = "user")]
#[belongs_to(Content, foreign_key = "parent")]
#[belongs_to(Category, foreign_key = "category")]
pub struct Content {
	pub id: i32,
	pub user: i32,
	pub created_at: chrono::NaiveDateTime,
	pub modified_at: chrono::NaiveDateTime,
	pub time: chrono::NaiveDateTime,
	pub title: Option<String>,
	pub slug: Option<String>,
	#[column_name = "content_"]
	pub content: String,
	pub draft_content: Option<String>,
	pub order_level: i32,
	#[column_name = "type_"]
	pub r#type: ContentType,
	pub status: ContentStatus,
	pub view_password: Option<String>,
	pub allow_comment: bool,
	pub allow_feed: bool,
	pub parent: Option<i32>,
	pub category: Option<i32>,
}
impl Content {
	last!(content);
	insert!(content, NewContent);
	find_pk!(content);
	find_one_by!(content, find_by_slug, slug as &str);
	update!();

	// -- general methods --
	pub fn count_post(db: &Database, status: &Vec<ContentStatus>) -> Result<i64> {
		content::table
			.filter(content::type_.eq(ContentType::Article))
			.filter(content::status.eq_any(status))
			.count()
			.get_result(&*db.pool().get()?)
			.map_err(Error::from)
	}

	pub fn find_posts(
		db: &Database,
		(min, max): (i32, i32),
		status: &Vec<ContentStatus>,
		sort_by_id: bool,
	) -> Result<Vec<Self>> {
		let mut query = content::table.into_boxed();

		query = query
			.filter(content::type_.eq(ContentType::Article))
			.filter(content::status.eq_any(status));

		query = if sort_by_id {
			query.order(content::id.desc())
		} else {
			query.order(content::time.desc())
		};
		query = query.offset(min.into()).limit((max - min).into());
		query.load::<Self>(&*db.pool().get()?).map_err(Error::from)
	}

	pub fn get_tags(&self, db: &Database) -> Result<Vec<Tag>> {
		let assocs = AssocTagContent::find_by_content_id(db, self.id)?;
		Tag::find_by_id(db, assocs.iter().map(|t| t.tag).collect::<Vec<i32>>())
	}

	pub fn set_tags(&self, db: &Database, tags: Vec<&str>) -> Result<()> {
		let tags = Tag::find_by_name(db, tags)?;
		AssocTagContent::update(db, self.id, tags)
	}

	pub fn get_tags_name(&self, db: &Database) -> Result<Vec<String>> {
		let tags = self
			.get_tags(db)?
			.iter()
			.map(|t| t.name.to_owned())
			.collect::<Vec<String>>();
		Ok(tags)
	}

	pub fn get_link(&self) -> String {
		let path = if let Some(slug) = &self.slug {
			slug.to_owned()
		} else {
			self.id.to_string()
		};
		uri!(crate::routes::post::post_show: path = format!("{}.html", path)).to_string()
	}

	pub fn get_category(&self, db: &Database) -> Result<Option<Category>> {
		if let Some(cid) = self.category {
			match Category::find(db, cid) {
				Ok(c) => Ok(Some(c)),
				Err(Error::NotFound) => Ok(None),
				Err(e) => Err(e),
			}
		} else {
			Ok(None)
		}
	}

	pub fn get_category_name(&self, db: &Database) -> Result<Option<String>> {
		if let Some(cat) = self.get_category(db)? {
			Ok(Some(cat.name.to_owned()))
		} else {
			Ok(None)
		}
	}

	pub fn get_user(&self, db: &Database) -> Result<User> {
		User::find(db, self.user)
	}

	pub fn user_has_access(&self, user: Option<&User>) -> bool {
		match user {
			Some(_) => self.status.is_visible_to_logged_in(),
			None => self.status.is_visible_to_public(),
		}
	}

	// -- methods for posts --
	/// is_prev: true-> prev_post, false-> next_post
	pub fn find_neighbor_post(
		&self,
		db: &Database,
		prev: bool,
		limit: i64,
	) -> Result<Option<Self>> {
		let mut query = content::table.into_boxed();

		query = query
			.filter(content::type_.eq(ContentType::Article))
			.filter(content::status.eq(ContentStatus::Normal));
		query = if prev {
			query
				.filter(content::id.ne(self.id))
				.filter(content::time.le(&self.time))
				.order((content::time.desc(), content::id.desc()))
		} else {
			query
				.filter(content::id.ne(self.id))
				.filter(content::time.ge(&self.time))
				.order((content::time.asc(), content::id.asc()))
		};

		query = query.limit(limit);

		match query.get_result::<Self>(&*db.pool().get()?) {
			Ok(v) => Ok(Some(v)),
			Err(diesel::result::Error::NotFound) => Ok(None),
			Err(e) => Err(Error::from(e)),
		}
	}
}

#[derive(Insertable, Debug)]
#[table_name = "content"]
pub struct NewContent {
	pub user: i32,
	pub time: chrono::NaiveDateTime,
	pub title: Option<String>,
	pub slug: Option<String>,
	#[column_name = "content_"]
	pub content: String,
	pub draft_content: Option<String>,
	pub order_level: i32,
	#[column_name = "type_"]
	pub r#type: ContentType,
	pub status: ContentStatus,
	pub view_password: Option<String>,
	pub allow_comment: bool,
	pub allow_feed: bool,
	pub parent: Option<i32>,
	pub category: Option<i32>,
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
pub enum ContentStatus {
	Normal = 0,         // Shows in list, visible to public
	Deleted = 1,        // deleted
	Hidden = 2,         // Not shown in list, visible to public
	Unpublished = 3,    // only shows in admin panel, not visible everywhere
	WithAccessOnly = 4, // [not implemented] shows in list and visible only if logged in.
}
impl ContentStatus {
	pub const PUBLIC_LIST: [Self; 1] = [Self::Normal];
	pub const LOGGED_IN_LIST: [Self; 2] = [Self::Normal, Self::WithAccessOnly];
	pub const ADMIN_LIST: [Self; 4] = [
		Self::Normal,
		Self::Hidden,
		Self::Unpublished,
		Self::WithAccessOnly,
	];
	pub const PUBLIC_VISIBLE: [Self; 2] = [Self::Normal, Self::Hidden];
	pub const LOGGED_IN_VISIBLE: [Self; 3] = [Self::Normal, Self::Hidden, Self::WithAccessOnly];

	pub fn is_visible_to_public(&self) -> bool {
		Self::PUBLIC_VISIBLE.contains(self)
	}

	pub fn is_visible_to_logged_in(&self) -> bool {
		Self::LOGGED_IN_VISIBLE.contains(self)
	}

	// not impl std::convert::TryFrom　for some reasons
	pub fn try_from(n: i32) -> Result<Self> {
		match n {
			0 => Ok(ContentStatus::Normal),
			1 => Ok(ContentStatus::Deleted),
			2 => Ok(ContentStatus::Hidden),
			3 => Ok(ContentStatus::Unpublished),
			4 => Ok(ContentStatus::WithAccessOnly),
			n => Err(Error::NoEnumNumber("ContentStatus".to_string(), n)),
		}
	}
}
impl FromSql<Integer, Mysql> for ContentStatus {
	fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
		let i = <i32 as FromSql<Integer, Mysql>>::from_sql(bytes)?;
		match Self::try_from(i) {
			Ok(s) => Ok(s),
			Err(_) => Err(format!("Failed convert `{}` to ContentStatus", i).into()),
		}
	}
}
impl ToSql<Integer, Mysql> for ContentStatus {
	fn to_sql<W: std::io::Write>(
		&self,
		out: &mut serialize::Output<W, Mysql>,
	) -> serialize::Result {
		ToSql::<Integer, Mysql>::to_sql(&(*self as i32), out)
	}
}
impl ToHtml for ContentStatus {
	fn to_html(&self, out: &mut dyn std::io::Write) -> std::io::Result<()> {
		write!(out, "{}", *self as i32)
	}
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug, FromSqlRow, AsExpression, PartialEq)]
#[repr(u8)]
#[serde(rename_all = "lowercase")]
#[sql_type = "Integer"]
pub enum ContentType {
	Article = 0,
	SinglePage = 1,
}
impl FromSql<Integer, Mysql> for ContentType {
	fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
		match <i32 as FromSql<Integer, Mysql>>::from_sql(bytes)? {
			0 => Ok(ContentType::Article),
			1 => Ok(ContentType::SinglePage),
			n => Err(format!("Unknown ContentType: {}", n).into()),
		}
	}
}
impl ToSql<Integer, Mysql> for ContentType {
	fn to_sql<W: std::io::Write>(
		&self,
		out: &mut serialize::Output<W, Mysql>,
	) -> serialize::Result {
		ToSql::<Integer, Mysql>::to_sql(&(*self as i32), out)
	}
}
