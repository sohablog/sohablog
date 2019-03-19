use rocket_codegen::*;
use rocket_contrib::{
	templates::Template
};
use rocket::State;
use crate::{
	db::Database,
	render,
	models::{
		user::{self,User},
		content::Content
	},
};
use super::super::error::Error;

#[get("/admin/post")]
pub fn list(db: State<Database>, global_var: render::GlobalVariable,current_user: User)->Result<Template,Error>{
	current_user.check_permission(user::PERM_POST_VIEW)?;
	let mut ctx=tera::Context::new();
	let posts=Content::find_posts(&db,(0,10),true)?;
	ctx.insert("posts",&posts);
	Ok(render::render("admin/post/list",global_var,Some(ctx))?)
}
