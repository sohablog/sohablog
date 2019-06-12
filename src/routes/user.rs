use crate::{
	models::user,
	render::RenderResult,
	util::*,
	templates,
};
use rocket::{
	http::{Cookie, Cookies},
	request::LenientForm,
	response::Redirect,
};
use rocket_codegen::*;

#[get("/user/login")]
pub fn login_get(gctx: GlobalContext) -> RenderResult {
	render!(templates::user::login, &gctx, None, None)
}

#[derive(Default, FromForm, Debug)]
pub struct LoginForm {
	pub username: String,
	pub password: String,
}
#[post("/user/login", data = "<form>")]
pub fn login_post(
	gctx: GlobalContext,
	mut cookies: Cookies,
	form: LenientForm<LoginForm>,
) -> Result<Redirect, RenderResult> {
	if let Ok(user) = user::User::find_by_username(&gctx.db, form.username.as_str()) {
		if user.verify_password_hash(form.password.as_str()) {
			cookies.add_private(Cookie::new("user_id", user.id.to_string()));
			return Ok(Redirect::to("/admin"));
		}
	}
	Err(render!(
		templates::user::login,
		&gctx,
		Some(String::from("Wrong username or password")),
		Some(String::from(&form.username))
	))
}
