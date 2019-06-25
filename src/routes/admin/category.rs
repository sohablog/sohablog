use super::super::error::Error;
use crate::{
	db::Database,
	models::{
		category::{Category, NewCategory},
		user::{self, User},
		IntoInterface
	},
	render::RenderResult,
	templates,
	util::*,
};
use rocket::{request::LenientForm, response::Redirect, State};
use rocket_codegen::*;

#[get("/admin/category")]
pub fn list(gctx: GlobalContext, current_user: User) -> Result<RenderResult, Error> {
	current_user.check_permission(user::PERM_CATEGORY_MANAGE)?;
	Ok(render!(
		templates::admin::category::list,
		&gctx.get_template_context(),
		Category::find_all(&gctx.db)?.into_interface(&gctx.db)
	))
}

#[derive(Default, FromForm, Debug)]
pub struct PostForm {
	pub slug: String,
	pub name: String,
	pub order: i32,
	pub parent: String,
	pub description: String,
	pub target: Option<i32>,
}
#[post("/admin/category/update", data = "<form>")]
pub fn update(
	db: State<Database>,
	form: LenientForm<PostForm>,
	current_user: User,
	_csrf: CSRFTokenValidation,
) -> Result<Redirect, Error> {
	current_user.check_permission(user::PERM_CATEGORY_MANAGE)?;

	let new_cat = NewCategory {
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
			let cat: Category = Category::find_by_slug(&db, &form.parent.trim())?;
			Some(cat.id)
		} else {
			None
		},
		description: if form.description.trim().len() > 0 {
			Some(form.description.trim().to_owned())
		} else {
			None
		},
		order: form.order,
	};
	match form.target {
		Some(id) => {
			let mut cat: Category = Category::find(&db, id)?;
			cat.slug = new_cat.slug;
			cat.name = new_cat.name;
			cat.parent = new_cat.parent;
			cat.description = new_cat.description;
			cat.order = new_cat.order;
			cat.update(&db)?;
		}
		None => {
			Category::insert(&db, new_cat)?;
		}
	};
	Ok(Redirect::to(uri!(list)))
}
