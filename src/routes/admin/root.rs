use super::super::error::Error;
use crate::{models::user::User, render, templates};
use rocket_codegen::*;

#[get("/admin")]
pub fn index(gctx: render::GlobalContext, _user: User) -> render::RenderResult {
	render!(templates::admin::index, &gctx)
}

#[get("/admin/generatePasswordHash?<p>")]
pub fn generate_password_hash(p: String) -> Result<String, Error> {
	Ok(User::generate_password_hash(p.as_str())?)
}
