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
	fn post_list(ctx: &TemplateContext, title: &str, page: Page, posts: Vec<Box<Content>>) -> RenderResult {
		let mut buf: Vec<u8> = vec![];
		let theme_name = &ctx.system_config.theme_name;
		RenderResult(buf)
	}
	fn post_show(ctx: &TemplateContext, title: &str, post: Box<Content>, previous_author: Option<Box<Author>>) -> RenderResult {
		let mut buf = vec![];
		RenderResult(buf)
	}
}
