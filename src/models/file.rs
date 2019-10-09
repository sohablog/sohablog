use super::{Error, Result};
use crate::{db::Database, schema::*};
use diesel::prelude::*;
use serde_derive::*;
use chrono::{DateTime, Local};

#[derive(Identifiable, Debug, Queryable, Clone, Serialize)]
#[primary_key(id)]
#[table_name = "file"]
pub struct File {
	pub id: i32,
	pub key: String,
	pub filename: String,
	pub content: Option<i32>,
	pub user: i32,
	pub time: DateTime<Local>,
}
#[derive(Insertable, Debug)]
#[table_name = "file"]
pub struct NewFile {
	pub key: String,
	pub filename: String,
	pub user: i32,
	pub content: Option<i32>,
}
impl File {
	last!(file);
	insert!(file, NewFile);
	find_pk!(file);
	delete!();
	find_by!(file, find_by_content_id, content as i32);
	find_by!(file, find_by_user_id, user as i32);

	pub fn new(key: String, filename: String, user_id: i32, content_id: Option<i32>) -> NewFile {
		NewFile {
			key: key,
			filename: filename,
			content: content_id,
			user: user_id,
		}
	}

	pub fn create(
		db: &Database,
		key: String,
		filename: String,
		user_id: i32,
		content_id: Option<i32>,
	) -> Result<Self> {
		Self::insert(db, Self::new(key, filename, user_id, content_id))
	}
}
