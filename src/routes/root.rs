use rocket_codegen::*;

use super::{error::Error, Page};
use crate::{
	models::content,
	render::{GlobalContext, RenderResult},
	theme::templates,
};

#[get("/?<page>")]
pub fn index(
	gctx: GlobalContext,
	mut page: Page,
) -> Result<RenderResult, Error> {
	let posts = content::Content::find_posts(&gctx.db, page.range(super::post::ITEMS_PER_PAGE), false)?;
	page.calc_total(
		content::Content::count_post(&gctx.db, false)? as i32,
		super::post::ITEMS_PER_PAGE,
	);
	
	Ok(render!(
		templates::post_list,
		&gctx,
		"Index",
		page,
		posts
	))
}

#[get("/<path>")]
pub fn page_show(
	gctx: GlobalContext,
	path: String,
) -> Result<RenderResult, Error> {
	let slug = path.replace(".html", ""); // TODO: We just need to remove `.html` at the end
	let post: content::Content = content::Content::find_by_slug(&gctx.db, &slug)?;
	if post.status == content::ContentStatus::Deleted
		|| post.r#type != content::ContentType::SinglePage
	{
		return Err(Error::NotFound);
	}
	// TODO: Password check when `view_password` exists

	Ok(render!(
		templates::post_show,
		&gctx,
		format!("{}", post.title.as_ref().unwrap_or(&String::from("Untitled"))).as_str(),
		post
	))
}
