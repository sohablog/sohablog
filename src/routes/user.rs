use rocket_codegen::*;

use rocket::{
	State,
	response::Redirect,
	request::LenientForm
};
use rocket_contrib::{
	templates::Template
};
use crate::db::Database;
use crate::models;

#[get("/login")]
pub fn login_get(db: State<Database>)->Template{
	let ctx=tera::Context::new();
	Template::render("user/login",&ctx)
}

#[derive(Default,FromForm,Debug)]
pub struct LoginForm {
	pub username: String,
	pub password: String
}

#[post("/login",data="<form>")]
pub fn login_post(db: State<Database>,form: LenientForm<LoginForm>)->String{
	let user:models::user::User=models::user::User::find_by_username(&db,form.username.as_str()).unwrap();
	let pwd_verify=user.verify_password_hash(form.password.as_str());
	format!("{:?}\n{:?}\n{:?}",form,user,pwd_verify)
}
