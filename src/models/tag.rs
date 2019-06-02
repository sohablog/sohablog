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
	find_one_by!(tag, find_by_name, name as &str);
	update!();
}

/* Things for associations between Tag and Content */
#[derive(Identifiable, Debug, Queryable, Associations)]
#[belongs_to(Tag, foreign_key = "tag")]
#[belongs_to(Content, foreign_key = "content")]
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
	last!(assoc_tag_content);
	insert!(assoc_tag_content, NewAssocTagContent);
	find_one_by!(tag, find_by_tag_id, tag as i32);
	find_one_by!(tag, find_by_content_id, content as i32);
}
