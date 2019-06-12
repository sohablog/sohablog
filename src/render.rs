use crate::{
	db::Database,
	models::user,
	routes::VisitorIP,
	SystemConfig
};
use comrak::{self, ComrakOptions};
use rocket::{
	http::uri::Origin,
	request::{FromRequest, Outcome},
	response::{self, Responder},
	Request, State,
};
use std::io::{Result as IoResult, Write};

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

pub trait ToHtml {
	fn to_html(&self, out: &mut dyn Write) -> IoResult<()>;
}
impl ToHtml for Origin<'_> {
	fn to_html(&self, out: &mut dyn Write) -> IoResult<()> {
		write!(out, "{}", &self.to_string())
	}
}

/// `GlobalContext` is a struct contained some globally useful items, such as user and database connection.
pub struct GlobalContext<'a> {
	pub ip: VisitorIP,
	pub db: State<'a, Database>,
	pub user: Option<user::User>,
	pub system_config: State<'a, SystemConfig>,
}
impl<'a, 'r> FromRequest<'a, 'r> for GlobalContext<'r> {
	type Error = ();
	fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
		Outcome::Success(Self {
			ip: request.guard::<VisitorIP>().unwrap(), // FIXME: Needs to process errors properly
			db: request.guard::<State<Database>>()?,
			user: request.guard::<Option<user::User>>().unwrap(),
			system_config: request.guard::<State<SystemConfig>>()?
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
pub fn markdown_to_html(out: &mut dyn Write, s: &str) -> IoResult<()> {
	let s = comrak::markdown_to_html(s, &COMRAK_OPTIONS);
	write!(out, "{}", s)
}

pub fn date_format(time: &chrono::NaiveDateTime, fmt: &str) -> String {
	time.format(fmt).to_string()
}
