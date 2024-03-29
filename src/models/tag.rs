use super::{Error, RepositoryWrapper, Result};
use crate::{db::Database, schema::*, utils::*};
use diesel::prelude::*;
use serde_derive::*;

#[derive(Identifiable, Debug, Queryable, Clone, Serialize)]
#[primary_key(id)]
#[table_name = "tag"]
pub struct Tag {
	pub id: i32,
	pub name: String,
}
#[derive(Insertable, Debug)]
#[table_name = "tag"]
pub struct NewTag {
	pub name: String,
}
impl Tag {
	insert!(tag, NewTag);
	find_pk!(tag);

	pub fn new(name: &str) -> NewTag {
		NewTag {
			name: name.trim().to_lowercase().to_string(),
		}
	}

	pub fn find_by_name(db: &Database, names: Vec<&str>) -> Result<Vec<Tag>> {
		let names: Vec<NewTag> = names.iter().map(|name| Self::new(name)).collect();
		let mut tags = tag::table
			.filter(tag::name.eq_any(names.iter().map(|t| t.name.as_str()).collect::<Vec<&str>>()))
			.load::<Self>(&db.conn()?)?;
		tags.extend(
			diesel::insert_into(tag::table).values(
				names.iter()
					.filter(|tag| !tags.iter().any(|t| t.name == tag.name))
					.collect::<Vec<&NewTag>>()
			).get_results(&db.conn()?)?
		);
		Ok(tags)
	}

	pub fn find_by_id(db: &Database, tags: Vec<i32>) -> Result<Vec<Tag>> {
		tag::table
			.into_boxed()
			.filter(tag::id.eq_any(tags))
			.load::<Self>(&db.conn()?)
			.map_err(Error::from)
	}
}

use crate::interfaces::models::Tag as TagInterface;
impl TagInterface for RepositoryWrapper<Tag, Box<Database>> {
	fn name(&self) -> &String {
		&self.0.name
	}
}
create_into_interface!(dyn TagInterface, Tag);

/* Things for associations between Tag and Content */
#[derive(Identifiable, Debug, Queryable, Associations)]
#[belongs_to(Tag, foreign_key = "tag")]
#[belongs_to(super::content::Content, foreign_key = "content")]
#[table_name = "assoc_tag_content"]
pub struct AssocTagContent {
	pub id: i32,
	pub tag: i32,
	pub content: i32,
}
#[derive(Insertable, Debug)]
#[table_name = "assoc_tag_content"]
pub struct NewAssocTagContent {
	pub tag: i32,
	pub content: i32,
}
impl AssocTagContent {
	pub fn find_by_content_id(db: &Database, content_id: i32) -> Result<Vec<Self>> {
		assoc_tag_content::table
			.into_boxed()
			.filter(assoc_tag_content::content.eq(content_id))
			.load::<Self>(&db.conn()?)
			.map_err(Error::from)
	}

	pub fn update(db: &Database, content_id: i32, tags: Vec<Tag>) -> Result<()> {
		let tag_ids: Vec<i32> = tags.iter().map(|t| t.id).collect();
		let exist_assocs: Vec<i32> = Self::find_by_content_id(db, content_id)?
			.iter()
			.map(|o| o.tag)
			.collect();
		let removing_ids: Vec<i32> = exist_assocs
			.iter()
			.cloned()
			.filter(|id| !tag_ids.contains(&id))
			.collect();
		let adding_objects: Vec<NewAssocTagContent> = tag_ids
			.iter()
			.filter(|&id| !exist_assocs.contains(id))
			.map(|&id| NewAssocTagContent {
				tag: id,
				content: content_id,
			})
			.collect();

		// first deletes tags which is not exists in this post
		diesel::delete(assoc_tag_content::table)
			.filter(assoc_tag_content::content.eq(content_id))
			.filter(assoc_tag_content::tag.eq_any(removing_ids))
			.execute(&db.conn()?)?;
		// then insert new added tags
		diesel::insert_into(assoc_tag_content::table)
			.values(&adding_objects)
			.execute(&db.conn()?)?;
		Ok(())
	}
}
