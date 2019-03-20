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
	let slug=path.replace(".html",""); // TODO: We just need to remove `.html` at the end
	let post=match slug.parse::<i32>(){
		Ok(post_id)=>content::Content::find(&db,post_id)?,
		Err(_)=>content::Content::find_by_slug(&db,&slug)?
	};
	if post.status==content::ContentStatus::Deleted || post.r#type!=content::ContentType::Article{
		return Err(Error::NotFound)
	}
	// TODO: Password check when `view_password` exists
	let poster=post.get_user(&db)?;
	let mut ctx=tera::Context::new();
	ctx.insert("post",&post);
	ctx.insert("poster",&poster);
	Ok(render::theme_render("post",global_var,Some(ctx))?)
}
