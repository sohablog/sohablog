use rocket_codegen::*;

use rocket::{
	State
};
use rocket_contrib::{
	templates::Template
};
use crate::db::Database;
use crate::models::content;
use super::error::Error;

#[get("/post/<path>")]
pub fn post_show(db: State<Database>,path: String)->Result<String,Error>{
	let slug=path.replace(".html","");
	let post=content::Content::find_by_slug(&db,&slug)?;
	Ok(format!("{:?}\n{:?}",slug,post))
}
