use rocket_codegen::*;

use rocket::{
	State
};
use rocket_contrib::{
	templates::Template
};
use crate::render;
use crate::db::Database;
use crate::models::content;
use super::error::Error;

#[get("/post/<path>")]
pub fn post_show(db: State<Database>,global_var: render::GlobalVariable,path: String)->Result<Template,Error>{
	let slug=path.replace(".html","");
	let post=content::Content::find_by_slug(&db,&slug)?;
	
	let mut ctx=tera::Context::new();
	ctx.insert("post",&post);
	Ok(render::theme_render("post",global_var,Some(ctx))?)
}
