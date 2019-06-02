use diesel::prelude::*;
use serde_derive::*;
use super::{Error, Result};
use crate::schema::*;

#[derive(Identifiable, Debug, Associations, Queryable, Clone, Serialize, AsChangeset)]
#[primary_key(id)]
#[table_name = "category"]
#[belongs_to(Category, foreign_key = "parent")]
pub struct Category {
	pub id: i32,
	pub slug: String,
	pub name: String,
	pub description: Option<String>,
	pub order: i32,
	pub parent: Option<i32>,
}
impl Category {
	last!(category);
	insert!(category, NewCategory);
	find_pk!(category);
	find_one_by!(category, find_by_name, name as &str);
	find_one_by!(category, find_by_slug, slug as &str);
	update!();

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

#[derive(Insertable, Debug)]
#[table_name = "category"]
pub struct NewCategory {
	pub slug: String,
	pub name: String,
	pub description: Option<String>,
	pub order: i32,
	pub parent: Option<i32>,
}
