use crate::{
	models::user,
	db::Database
};
use comrak::{self, ComrakOptions};
use rocket::{
	http::uri::Origin,
	request::{FromRequest, Outcome},
	response::{self, Responder},
	Request,
	State,
};
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::result::Result;
use tera::*;
use std::io::{
	Write,
	Result as IoResult
};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
	TemplateRender,
}

/// `RenderResult` wraps a Vec<u8> which is the HTML render result.
#[derive(Debug)]
pub struct RenderResult(pub Vec<u8>);
impl<'r> Responder<'r> for RenderResult {
	fn respond_to(self, req: &Request) -> response::Result<'r> {
		response::content::Html(self.0).respond_to(req)
	}
}

/// returns `RenderResult`
#[macro_export]
macro_rules! render {
	($path:path, $($param:expr),*) => {{
		use crate::render::RenderResult;

		let mut buf = vec![];
		$path(&mut buf,$($param),*).unwrap();
		RenderResult(buf)
	}}
}

trait ToHtml {
	fn to_html(&self, out: &mut Write) -> IoResult<()>;
}
impl ToHtml for Origin<'_> {
	fn to_html(&self, out: &mut Write) -> IoResult<()> {
		write!(out, "{}", &self.to_string())
	}
}

/// `GlobalContext` is a struct contained some globally useful items, such as user and database connection.
pub struct GlobalContext<'a> {
	pub db: State<'a, Database>,
	pub user: Option<user::User>
}
impl<'a, 'r> FromRequest<'a, 'r> for GlobalContext<'r> {
	type Error = ();
	fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
		Outcome::Success(Self {
			db: request.guard::<State<Database>>()?,
			user: request.guard::<Option<user::User>>().unwrap()
		})
	}
}

/// Options for `comrak` which is a Markdown parser
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
/// Parses markdown to HTML
pub fn markdown_to_html(out: &mut Write, s: &str) -> IoResult<()> {
	let s = comrak::markdown_to_html(s, &COMRAK_OPTIONS);
	write!(out, "{}", s)
}

#[deprecated]
fn create_final_context(global_var: GlobalVariable, context: Option<Context>) -> Context {
	let mut ctx = Context::new();
	ctx.insert("currentUser", &global_var.current_user);
	if let Some(context) = context {
		ctx.extend(context);
	};
	ctx
}

#[deprecated]
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

#[deprecated]
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

#[deprecated]
pub fn tera_filter_markdown(
	value: tera::Value,
	_: HashMap<String, tera::Value>,
) -> tera::Result<tera::Value> {
	let s = try_get_value!("markdown", "value", String, value);
	let html = comrak::markdown_to_html(s.as_str(), &COMRAK_OPTIONS);
	Ok(to_value(html).unwrap())
}
