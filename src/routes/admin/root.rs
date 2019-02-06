use rocket_codegen::*;
use rocket_contrib::{
	templates::Template
};
use crate::models::{user::User};

#[get("/admin")]
pub fn index(user: User)->Template{
	let mut ctx=tera::Context::new();
	ctx.insert("currentUser",&user);
	Template::render("admin/index",&ctx)
}
