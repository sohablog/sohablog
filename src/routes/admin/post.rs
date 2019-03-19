use rocket_codegen::*;
use rocket_contrib::{
	templates::Template
};
use rocket::{
	State,
	request::LenientForm
};
use crate::{
	db::Database,
	render,
	models::{
		user::{self,User},
		content::{self,Content}
	},
};
use super::super::error::Error;

#[get("/admin/post")]
pub fn list(db: State<Database>,global_var: render::GlobalVariable,current_user: User)->Result<Template,Error>{
	current_user.check_permission(user::PERM_POST_VIEW)?;
	let mut ctx=tera::Context::new();
	let posts=Content::find_posts(&db,(0,10),true)?;
	ctx.insert("posts",&posts);
	Ok(render::render("admin/post/list",global_var,Some(ctx))?)
}

#[get("/admin/post/_new")]
pub fn new_get(global_var: render::GlobalVariable,current_user: User)->Result<Template,Error>{
	current_user.check_permission(user::PERM_POST_EDIT)?;
	Ok(render::render("admin/post/edit",global_var,None)?)
}
#[get("/admin/post/<post_id>")]
pub fn edit_get(post_id:i32,db: State<Database>,global_var: render::GlobalVariable,current_user: User)->Result<Template,Error>{
	current_user.check_permission(user::PERM_POST_EDIT)?;
	let post:Content=Content::find(&db,post_id)?;
	let mut ctx=tera::Context::new();
	if post.status==content::ContentStatus::Deleted || post.r#type!=content::ContentType::Article{
		return Err(Error::NotFound)
	}
	ctx.insert("post",&post);
	Ok(render::render("admin/post/edit",global_var,Some(ctx))?)
}
#[derive(Default,FromForm,Debug)]
pub struct LoginForm {
	pub username: String,
	pub password: String
}
#[post("/admin/post/_edit",data="<form>")]
pub fn edit_post(db: State<Database>,form: LenientForm<LoginForm>)->Result<Template,Template>{
	let mut ctx=tera::Context::new();
	let mut error=tera::Context::new();
	error.insert("message","Wrong username or password");
	ctx.insert("error",&error);
	ctx.insert("username",&form.username);
	Err(Template::render("admin/user/login",&ctx))
}
