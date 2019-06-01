use crate::models::user;
use comrak::{markdown_to_html, ComrakOptions};
use rocket::{
	request::{FromRequest, Outcome},
	Request,
};
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::result::Result;
use tera::*;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
	TemplateRender,
}

fn create_final_context(global_var: GlobalVariable, context: Option<Context>) -> Context {
	let mut ctx = Context::new();
	ctx.insert("currentUser", &global_var.current_user);
	if let Some(context) = context {
		ctx.extend(context);
	};
	ctx
}

pub fn theme_render(
	name: &str,
	global_var: GlobalVariable,
	context: Option<Context>,
) -> Result<Template, Error> {
	let ctx = create_final_context(global_var, context);
	Ok(Template::render(
		format!("theme/{}/{}", "basic", name),
		&ctx,
	))
}

pub fn render(
	name: &'static str,
	global_var: GlobalVariable,
	context: Option<Context>,
) -> Result<Template, Error> {
	let ctx = create_final_context(global_var, context);
	Ok(Template::render(name, &ctx))
}

pub struct GlobalVariable {
	pub current_user: Option<user::User>,
}
impl<'a, 'r> FromRequest<'a, 'r> for GlobalVariable {
	type Error = ();
	fn from_request(request: &'a Request<'r>) -> Outcome<GlobalVariable, ()> {
		let user = request.guard::<Option<user::User>>().unwrap();
		Outcome::Success(GlobalVariable { current_user: user })
	}
}

const COMRAK_OPTIONS: ComrakOptions = ComrakOptions {
	hardbreaks: false,
	smart: true,
	github_pre_lang: true,
	width: 0,
	default_info_string: None,
	unsafe_: true,
	ext_strikethrough: true,
	ext_tagfilter: true,
	ext_table: true,
	ext_autolink: true,
	ext_tasklist: true,
	ext_superscript: true,
	ext_header_ids: None,
	ext_footnotes: true,
	ext_description_lists: true,
};

pub fn tera_filter_markdown(
	value: tera::Value,
	_: HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
	let s = try_get_value!("markdown", "value", String, value);
	let html = markdown_to_html(s.as_str(), &COMRAK_OPTIONS);
	Ok(to_value(html).unwrap())
}
