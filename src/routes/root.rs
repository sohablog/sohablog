use super::{error::Error, Page};
use crate::{
	models::{comment::Author, content, IntoInterface},
	render::{theme, RenderResult},
	util::*,
};
use rocket::http::Cookies;
use rocket_codegen::*;

#[get("/?<page>")]
pub fn index(gctx: GlobalContext, mut page: Page) -> Result<RenderResult, Error> {
	let post_status = if let None = gctx.user {
		content::ContentStatus::PUBLIC_LIST.to_vec()
	} else {
		content::ContentStatus::LOGGED_IN_LIST.to_vec()
	};
	let posts = content::Content::find_posts(
		&gctx.db,
		page.range(super::post::ITEMS_PER_PAGE),
		&post_status,
		false,
	)?;
	page.calc_total(
		content::Content::count_post(&gctx.db, &post_status)? as i32,
		super::post::ITEMS_PER_PAGE,
	);

	Ok(theme::post_list(
		&gctx,
		"Index",
		page,
		posts.into_interface(&gctx.db),
	)?)
}

#[get("/<path>")]
pub fn page_show(
	gctx: GlobalContext,
	mut cookies: Cookies,
	path: String,
) -> Result<RenderResult, Error> {
	let slug = path.replace(".html", ""); // TODO: We just need to remove `.html` at the end
	let post: content::Content = content::Content::find_by_slug(&gctx.db, &slug)?;
	if post.status == content::ContentStatus::Deleted
		|| post.r#type != content::ContentType::SinglePage
	{
		return Err(Error::NotFound);
	}
	if !post.user_has_access(gctx.user.as_ref()) {
		return Err(Error::PermissionDenied);
	}
	// TODO: Password check when `view_password` exists

	let previous_author = cookies
		.get_private("comment_author")
		.and_then(|c| serde_json::from_str::<Author>(c.value()).ok());
	Ok(theme::post_show(
		&gctx,
		format!(
			"{}",
			post.title.as_ref().unwrap_or(&String::from("Untitled"))
		)
		.as_str(),
		post.into_interface(&gctx.db),
		previous_author.into_interface(&gctx.db),
	)?)
}
