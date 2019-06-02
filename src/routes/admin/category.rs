use super::super::error::Error;
use crate::{
	db::Database,
	models::{
		user::{self, User},
		category::{self, Category}
	},
	render,
};
use rocket::{request::LenientForm, response::Redirect, State};
use rocket_codegen::*;
use rocket_contrib::templates::Template;

#[get("/admin/category")]
pub fn list(
	db: State<Database>,
	global_var: render::GlobalVariable,
	current_user: User,
) -> Result<Template, Error> {
	current_user.check_permission(user::PERM_CATEGORY_MANAGE)?;
	let mut ctx = tera::Context::new();
	let cats = Category::find_all(&db)?;
	ctx.insert("categories", &cats);
	Ok(render::render("admin/category/list", global_var, Some(ctx))?)
}
