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

#[derive(Default, FromForm, Debug)]
pub struct PostForm {
	pub slug: String,
	pub name: String,
	pub order: i32,
	pub parent: String,
	pub description: String,
	pub target: Option<String>,
}
#[post("/admin/category/update", data = "<form>")]
pub fn update(
	db: State<Database>,
	form: LenientForm<PostForm>,
	current_user: User,
) -> Result<Redirect, Error> {
	current_user.check_permission(user::PERM_CATEGORY_MANAGE)?;

	let new_cat = Category {
		slug: if form.slug.trim().len() == 0 {
			return Err(Error::BadRequest("`slug` field is illegal."));
		} else {
			form.slug.trim().to_owned()
		},
		name: if form.name.trim().len() == 0 {
			return Err(Error::BadRequest("`name` field is illegal."));
		} else {
			form.name.trim().to_owned()
		},
		parent: if form.parent.trim().len() > 0 {
			Some(form.parent.trim().to_owned())
		} else { None },
		description: if form.description.trim().len() > 0 {
			Some(form.description.trim().to_owned())
		} else { None },
		order: form.order 
	};
	
	match &form.target {
		Some(slug) => {
			let cat:Category = Category::find(&db, slug.as_str())?;
			cat.replace(&db, &new_cat)?;
		}
		None => {
			Category::insert(&db, new_cat)?;
		}
	};
	Ok(Redirect::to(uri!(list)))
}
