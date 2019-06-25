use super::{Error, Result, RepositoryWrapper};
use crate::{db::Database, utils::*, schema::*};
use diesel::prelude::*;
use serde_derive::*;

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

	pub fn find_all(db: &Database) -> Result<Vec<Self>> {
		let mut query = category::table.into_boxed();
		query = query.order(category::order.desc());
		query.load::<Self>(&db.conn()?).map_err(Error::from)
	}
}
impl PartialEq for Category {
	fn eq(&self, other: &Self) -> bool {
		self.slug == other.slug
	}
}

use crate::template_interfaces::models::Category as CategoryInterface;
impl CategoryInterface for RepositoryWrapper<Category, &'static Database> {
	fn slug(&self) -> &String { &self.0.slug }
	fn name(&self) -> &String { &self.0.name }
	fn description(&self) -> Option<&String> { self.0.description.as_ref() }
	fn order(&self) -> i32 { self.0.order }
	fn parent(&self) -> Option<i32> { self.0.parent }
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
