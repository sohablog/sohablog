use crate::{models::user, render::RenderResult, templates, util::*};
use rocket::{http::Cookies, request::LenientForm, response::Redirect};
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
	mut gctx: GlobalContext,
	_csrf: CSRFTokenValidation,
	mut cookies: Cookies,
	form: LenientForm<LoginForm>,
) -> Result<Redirect, RenderResult> {
	if let Ok(user) = user::User::find_by_username(&gctx.db, form.username.as_str()) {
		if user.verify_password_hash(form.password.as_str()) {
			gctx.session_info.user = Some(user.to_session_info());
			gctx.session_info.persist(&mut cookies, &gctx.system_config);
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
