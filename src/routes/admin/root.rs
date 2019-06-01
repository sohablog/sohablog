use super::super::error::Error;
use crate::{models::user::User, render};
use rocket_codegen::*;
use rocket_contrib::templates::Template;

#[get("/admin")]
pub fn index(global_var: render::GlobalVariable, _user: User) -> Result<Template, Error> {
	Ok(render::render("admin/index", global_var, None)?)
}

#[get("/admin/generatePasswordHash?<p>")]
pub fn generate_password_hash(p: String) -> Result<String, Error> {
	Ok(User::generate_password_hash(p.as_str())?)
}
