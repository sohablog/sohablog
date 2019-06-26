pub use crate::{types::EnumType, utils::TemplateContext};
use std::io::{Result as IoResult, Write};

#[cfg(feature = "main")]
use rocket::{
	http::uri::Origin,
	request::Request,
	response::{self, Responder},
};

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
	TemplateRender,
}

/// `RenderResult` wraps a Vec<u8> which is the HTML render result.
#[derive(Debug)]
pub struct RenderResult(pub Vec<u8>);
#[cfg(feature = "main")]
impl<'r> Responder<'r> for RenderResult {
	fn respond_to(self, req: &Request) -> response::Result<'r> {
		response::content::Html(self.0).respond_to(req)
	}
}

pub trait ToHtml {
	fn to_html(&self, out: &mut dyn Write) -> IoResult<()>;
}
#[cfg(feature = "main")]
impl ToHtml for Origin<'_> {
	fn to_html(&self, out: &mut dyn Write) -> IoResult<()> {
		write!(out, "{}", &self.to_string())
	}
}

pub trait RenderHelper {
	fn markdown_to_html(&self, s: &str) -> String;
	fn nl2br(&self, s: &str) -> String;
	fn date_format(&self, time: &chrono::NaiveDateTime, fmt: &str) -> String;
}
