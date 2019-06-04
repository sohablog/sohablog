use rocket_codegen::*;

use crate::{
	db::Database,
	models::user,
	render::{self, GlobalContext},
	templates
};
use rocket::{
	http::{Cookie, Cookies},
	request::LenientForm,
	response::Redirect,
	State,
};
use rocket_contrib::templates::Template;

#[get("/user/login")]
pub fn login_get(gctx: GlobalContext) -> render::RenderResult {
	render!(templates::user::login, gctx, None)
}

#[derive(Default, FromForm, Debug)]
pub struct LoginForm {
	pub username: String,
	pub password: String,
}
#[post("/user/login", data = "<form>")]
pub fn login_post(
	db: State<Database>,
	mut cookies: Cookies,
	form: LenientForm<LoginForm>,
) -> Result<Redirect, Template> {
	if let Ok(user) = user::User::find_by_username(&db, form.username.as_str()) {
		if user.verify_password_hash(form.password.as_str()) {
			cookies.add_private(Cookie::new("user_id", user.id.to_string()));
			return Ok(Redirect::to("/admin"));
		}
	}
	let mut ctx = tera::Context::new();
	let mut error = tera::Context::new();
	error.insert("message", "Wrong username or password");
	ctx.insert("error", &error);
	ctx.insert("username", &form.username);
	Err(Template::render("admin/user/login", &ctx))
}
