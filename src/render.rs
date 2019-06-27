pub use sohablog_lib::render::*;
use comrak::{self, ComrakOptions};
use std::io::{Result as IoResult, Write};

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

#[derive(Default, Debug)]
pub struct RenderFunctions;
impl RenderHelper for RenderFunctions {
	fn markdown_to_html(&self, s: &str) -> String {
		comrak::markdown_to_html(s, &COMRAK_OPTIONS)
	}
	fn nl2br(&self, s: &str) -> String {
		s.replace("\r\n", "\n").replace("\r", "\n").replace("\n", "<br />")
	}
	fn date_format(&self, time: &chrono::NaiveDateTime, fmt: &str) -> String {
		time.format(fmt).to_string()
	}
	fn truncate(&self, s: &str, len: usize) -> String {
		String::from(match s.char_indices().nth(len) {
			None => s,
			Some((idx, _)) => &s[..idx],
		})
	}
	fn truncate_content(&self, s: &str, len: usize, truncate_mark: bool) -> String {
		if truncate_mark {
			let v: Vec<&str> = s.split(CONTENT_TRUNCATE_MARK).collect();
			if v.len() > 1 {
				return self.markdown_to_html(v[0])
			}
		}
		self.nl2br(
			&self.truncate(
				&ammonia::Builder::new()
					.tags(std::collections::HashSet::new())
					.clean(&self.markdown_to_html(s))
					.to_string(),
				len
			)
		)
	}
}

/// call wrapped function and write them as HTML
pub fn markdown_to_html(out: &mut dyn Write, ctx: &TemplateContext, s: &str) -> IoResult<()> {
	let s = ctx.render_helper.markdown_to_html(s);
	write!(out, "{}", s)
}

pub fn nl2br(out: &mut dyn Write, ctx: &TemplateContext, s: &str) -> IoResult<()> {
	let s = ctx.render_helper.nl2br(s);
	write!(out, "{}", s)
}

pub fn truncate_content(out: &mut dyn Write, ctx: &TemplateContext, s: &str, len: usize, truncate_mark: bool) -> IoResult<()> {
	let s = ctx.render_helper.truncate_content(s, len, truncate_mark);
	write!(out, "{}", s)
}

/// returns `RenderResult`
#[macro_export]
macro_rules! render {
	($path:path, $($param:expr),*) => {{
		use sohablog_lib::render::RenderResult;

		let mut buf = vec![];
		$path(&mut buf,$($param),*).unwrap();
		RenderResult(buf)
	}}
}

pub mod theme {
	use crate::{theme::templates, util::GlobalContext};
	use sohablog_lib::{
		interfaces::models::{Author, Content},
		render::RenderResult,
		utils::{Page, StaticFile, TemplateContext},
	};
	use std::io::Result;

	pub fn post_list(
		ctx: &GlobalContext,
		title: &str,
		page: Page,
		posts: Vec<Box<dyn Content>>,
	) -> Result<RenderResult> {
		let theme_name = &ctx.system_config.theme_name;
		let theme_context: TemplateContext = ctx.get_template_context();
		Ok(
			if let Some(theme) = &ctx.plugin_manager.get_theme(theme_name) {
				let mut buf: Vec<u8> = vec![];
				theme.post_list(&mut buf, &theme_context, title, page, posts)?;
				RenderResult(buf)
			} else {
				render!(templates::post_list, &theme_context, title, page, posts)
			},
		)
	}

	pub fn post_show(
		ctx: &GlobalContext,
		title: &str,
		post: Box<dyn Content>,
		previous_author: Option<Box<dyn Author>>,
	) -> Result<RenderResult> {
		let theme_name = &ctx.system_config.theme_name;
		let theme_context: TemplateContext = ctx.get_template_context();
		Ok(
			if let Some(theme) = &ctx.plugin_manager.get_theme(theme_name) {
				let mut buf: Vec<u8> = vec![];
				theme.post_show(&mut buf, &theme_context, title, post, previous_author)?;
				RenderResult(buf)
			} else {
				render!(
					templates::post_show,
					&theme_context,
					title,
					post,
					previous_author
				)
			},
		)
	}

	pub fn get_static(ctx: &GlobalContext, name: &str) -> Option<Box<dyn StaticFile>> {
		let theme_name = &ctx.system_config.theme_name;
		if let Some(theme) = &ctx.plugin_manager.get_theme(theme_name) {
			theme.static_file(name)
		} else if let Some(f) = templates::statics::StaticFile::get(name) {
			Some(Box::new(f) as Box<dyn StaticFile>)
		} else {
			None
		}
	}
}
