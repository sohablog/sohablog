pub use crate::util::GlobalContext;
use comrak::{self, ComrakOptions};
use std::io::{Result as IoResult, Write};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
	TemplateRender,
}

/// `RenderResult` wraps a Vec<u8> which is the HTML render result.
#[derive(Debug)]
pub struct RenderResult(pub Vec<u8>);

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

pub fn nl2br(out: &mut dyn Write, s: &str) -> IoResult<()> {
	let s = s.replace("\r\n", "\n").replace("\r", "\n").replace("\n", "<br />");
	write!(out, "{}", s)
}

pub fn date_format(time: &chrono::NaiveDateTime, fmt: &str) -> String {
	time.format(fmt).to_string()
}
