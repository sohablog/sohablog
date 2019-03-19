use rocket_codegen::*;
use rocket_contrib::{
	templates::Template
};
use crate::{
	render,
	models::{user::User}
};
use super::super::error::Error;

#[get("/admin")]
pub fn index(global_var: render::GlobalVariable,_user: User)->Result<Template,Error>{
	Ok(render::render("admin/index",global_var,None)?)
}
