pub use sohablog_lib::render::*;

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
	use sohablog_lib::{
		render::RenderResult,
		utils::{Page, TemplateContext},
		interfaces::models::{Content, Author},
	};
	use crate::{
		util::GlobalContext,
		theme::templates,
	};
	use std::io::Result;

	pub fn post_list(ctx: &GlobalContext, title: &str, page: Page, posts: Vec<Box<Content>>) -> Result<RenderResult> {
		let theme_name = &ctx.system_config.theme_name;
		let theme_context: TemplateContext = ctx.get_template_context();
		Ok(if let Some(theme) = &ctx.plugin_manager.get_theme(theme_name) {
			let mut buf: Vec<u8> = vec![];
			theme.post_list(&mut buf, &theme_context, title, page, posts)?;
			RenderResult(buf)
		} else {
			render!(
				templates::post_list,
				&theme_context,
				title,
				page,
				posts
			)
		})
	}

	pub fn post_show(ctx: &GlobalContext, title: &str, post: Box<Content>, previous_author: Option<Box<Author>>) -> Result<RenderResult> {
		let theme_name = &ctx.system_config.theme_name;
		let theme_context: TemplateContext = ctx.get_template_context();
		Ok(if let Some(theme) = &ctx.plugin_manager.get_theme(theme_name) {
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
		})
	}
}
