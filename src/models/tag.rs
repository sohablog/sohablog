use diesel::prelude::*;
use serde_derive::*;
use super::{Error, Result};
use crate::schema::*;

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
	last!(tag);
	insert!(tag, NewTag);
	find_pk!(tag);

	pub fn new(name: String) -> NewTag {
		NewTag {
			name: name.to_lowercase()
		}
	}

	pub fn find_by_name(db: &crate::db::Database, tags: Vec<&str>) -> Result<Vec<Tag>> {
		let tags = tags.iter().filter(|&s| s.len() > 0).map(|&s| Self::new(s.to_string())).collect::<Vec<NewTag>>();
		diesel::insert_or_ignore_into(tag::table)
			.values(&tags)
			.execute(&*db.pool().get()?)?;
		tag::table.into_boxed()
			.filter(tag::name.eq_any(tags.iter().map(|t| t.name.as_str()).collect::<Vec<&str>>()))
			.load::<Self>(&*db.pool().get()?).map_err(Error::from)
	}

	pub fn find_by_id(db: &crate::db::Database, tags: Vec<i32>) -> Result<Vec<Tag>> {
		tag::table.into_boxed()
			.filter(tag::id.eq_any(tags))
			.load::<Self>(&*db.pool().get()?).map_err(Error::from)
	}
}

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
	pub fn find_by_content_id(db: &crate::db::Database, content_id: i32) -> Result<Vec<Self>> {
		assoc_tag_content::table.into_boxed()
			.filter(assoc_tag_content::content.eq(content_id))
			.load::<Self>(&*db.pool().get()?).map_err(Error::from)
	}

	pub fn update(db: &crate::db::Database, content_id: i32, tags: Vec<Tag>) -> Result<()> {
		let tag_ids = tags.iter().map(|t| t.id).collect::<Vec<i32>>();

		let exist_assocs = Self::find_by_content_id(db, content_id)?;
		let exist_assocs:Vec<i32> = exist_assocs.iter().map(|o| o.tag).collect();
		
		let mut removing_ids:Vec<i32> = Vec::new();
		for exist_tag_id in exist_assocs {
			if !tag_ids.contains(&exist_tag_id) {
				removing_ids.push(exist_tag_id);
			}
		}
		diesel::delete(assoc_tag_content::table)
			.filter(assoc_tag_content::content.eq(content_id))
			.filter(assoc_tag_content::tag.eq_any(removing_ids))
			.execute(&*db.pool().get()?)?;
		diesel::insert_or_ignore_into(assoc_tag_content::table)
			.values(
				tag_ids.iter().map(|&id| NewAssocTagContent {
					tag: id,
					content: content_id
				}).collect::<Vec<NewAssocTagContent>>()
			)
			.execute(&*db.pool().get()?)?;
		Ok(())
	}
}
