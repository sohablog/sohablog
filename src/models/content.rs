use diesel::prelude::*;
use serde_derive::*;

use crate::schema::*;
use crate::db::Database;
use super::{
	Error,
	Result,
	user::User
};

#[derive(Debug,Queryable,Associations,Clone,Serialize,Identifiable,AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name="content"]
#[primary_key(id)]
#[belongs_to(User,foreign_key="user")]
#[belongs_to(Content,foreign_key="parent")]
pub struct Content{
	pub id: i32,
	pub user: i32,
	pub created_at: chrono::NaiveDateTime,
	pub modified_at: chrono::NaiveDateTime,
	pub time: chrono::NaiveDateTime,
	pub title: Option<String>,
	pub slug: Option<String>,
	#[column_name="content_"]
	pub content: String,
	pub order_level: i32,
	#[column_name="type_"]
	pub r#type: ContentType,
	pub status: ContentStatus,
	pub view_password: Option<String>,
	pub allow_comment: bool,
	pub allow_feed: bool,
	pub parent: Option<i32>
}
impl Content{
	insert!(content,NewContent);
	find_pk!(content);
	find_one_by!(content,find_by_slug,slug as &str);
	update!();

	pub fn get_user(&self,db: &Database)->Result<User>{
		User::find(db,self.user)
	}

	pub fn find_posts(db: &crate::db::Database,(min,max):(i32,i32),with_hidden:bool)->Result<Vec<Self>>{
		let mut query=content::table.into_boxed();

		let mut status=vec![ContentStatus::Normal];
		if let true=with_hidden{
			status.push(ContentStatus::Hidden);
		}

		query=query
			.filter(content::type_.eq(ContentType::Article))
			.filter(content::status.eq_any(status));

		query=query.order(content::time.desc());
		query=query.offset(min.into()).limit((max-min).into());
		query.load::<Self>(&*db.pool().get()?).map_err(Error::from)
	}

	pub fn find_prev_post(db: &crate::db::Database,post: &Content)->Result<Option<Self>>{
		let mut query=content::table.into_boxed();

		query=query
			.filter(content::type_.eq(ContentType::Article))
			.filter(content::status.eq(ContentStatus::Normal))
			.filter(content::id.ne(post.id))
			.filter(content::time.le(&post.time));

		query=query.order((content::time.desc(),content::id.desc())).limit(1);

		match query.get_result::<Self>(&*db.pool().get()?){
			Ok(v)=>Ok(Some(v)),
			Err(diesel::result::Error::NotFound)=>Ok(None),
			Err(e)=>Err(Error::from(e))
		}
	}

	pub fn find_next_post(db: &crate::db::Database,post: &Content)->Result<Option<Self>>{
		let mut query=content::table.into_boxed();

		query=query
			.filter(content::type_.eq(ContentType::Article))
			.filter(content::status.eq(ContentStatus::Normal))
			.filter(content::id.ne(post.id))
			.filter(content::time.ge(&post.time));

		query=query.order((content::time.asc(),content::id.asc())).limit(1);

		match query.get_result::<Self>(&*db.pool().get()?){
			Ok(v)=>Ok(Some(v)),
			Err(diesel::result::Error::NotFound)=>Ok(None),
			Err(e)=>Err(Error::from(e))
		}
	}
}

#[derive(Insertable,Debug)]
#[table_name="content"]
pub struct NewContent{
	pub user: i32,
	pub time: chrono::NaiveDateTime,
	pub title: Option<String>,
	pub slug: Option<String>,
	#[column_name="content_"]
	pub content: String,
	pub order_level: i32,
	#[column_name="type_"]
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
	serialize::{
		ToSql,
		self
	},
	sql_types::Integer,
	mysql::Mysql,
};

#[derive(Copy,Clone,Serialize,Deserialize,Debug,FromSqlRow,AsExpression,PartialEq)]
#[repr(u8)]
#[sql_type="Integer"]
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
impl ToSql<Integer,Mysql> for ContentStatus{
	fn to_sql<W:std::io::Write>(&self,out: &mut serialize::Output<W,Mysql>)->serialize::Result{
		ToSql::<Integer,Mysql>::to_sql(&(*self as i32),out)
	}
}

#[derive(Copy,Clone,Serialize,Deserialize,Debug,FromSqlRow,AsExpression,PartialEq)]
#[repr(u8)]
#[serde(rename_all="lowercase")]
#[sql_type="Integer"]
pub enum ContentType{
	Article=0,
	Draft=1,
	SinglePage=2
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
impl ToSql<Integer,Mysql> for ContentType{
	fn to_sql<W:std::io::Write>(&self,out: &mut serialize::Output<W,Mysql>)->serialize::Result{
		ToSql::<Integer,Mysql>::to_sql(&(*self as i32),out)
	}
}
