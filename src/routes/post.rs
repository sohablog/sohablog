use super::error::Error;
use crate::{
	models::{comment::Author, content, IntoInterface},
	render::{theme, RenderResult},
	util::*,
};
use rocket::http::Cookies;
use rocket_codegen::*;

pub const ITEMS_PER_PAGE: i32 = 15;

#[get("/post/<path>")]
pub fn post_show(
	gctx: GlobalContext,
	mut cookies: Cookies,
	path: String,
) -> Result<RenderResult, Error> {
	let slug = path.replace(".html", ""); // TODO: We just need to remove `.html` at the end
	let post: content::Content = match slug.parse::<i32>() {
		Ok(post_id) => content::Content::find(&gctx.db, post_id)?,
		Err(_) => content::Content::find_by_slug(&gctx.db, &slug)?,
	};
	if post.status == content::ContentStatus::Deleted
		|| post.r#type != content::ContentType::Article
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
