use diesel::prelude::*;
use serde_derive::*;
use super::{Error, Result};
use crate::schema::*;

#[derive(Identifiable, Debug, Associations, Queryable, Clone, Serialize)]
#[primary_key(slug)]
#[table_name = "category"]
#[belongs_to(Category, foreign_key = "parent")]
pub struct Category {
	pub slug: String,
	pub name: String,
	pub description: Option<String>,
	pub order: i32,
	pub parent: Option<String>,
}
impl Category {
	find_pk!(category, slug as &str);
	insert_non_incremental!(category, NewCategory, slug);
	find_one_by!(category, find_by_name, name as &str);

	pub fn find_all(db: &crate::db::Database) -> Result<Vec<Self>> {
		let mut query = category::table.into_boxed();
		query = query.order(category::order.desc());
		query.load::<Self>(&*db.pool().get()?).map_err(Error::from)
	}
}
impl PartialEq for Category {
	fn eq(&self, other: &Self) -> bool {
		self.slug == other.slug
	}
}

#[derive(Insertable)]
#[table_name = "category"]
pub struct NewCategory {
	pub slug: String,
	pub name: String,
	pub description: Option<String>,
	pub parent: Option<String>,
}
