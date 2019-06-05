use rocket_codegen::*;

use super::error::Error;
use crate::{
	models::content,
	render::{GlobalContext, RenderResult},
	theme::templates,
};

pub const ITEMS_PER_PAGE: i32 = 15;

#[get("/post/<path>")]
pub fn post_show(
	gctx: GlobalContext,
	path: String,
) -> Result<RenderResult, Error> {
	let slug = path.replace(".html", ""); // TODO: We just need to remove `.html` at the end
	let post = match slug.parse::<i32>() {
		Ok(post_id) => content::Content::find(&gctx.db, post_id)?,
		Err(_) => content::Content::find_by_slug(&gctx.db, &slug)?,
	};
	if post.status == content::ContentStatus::Deleted
		|| post.r#type != content::ContentType::Article
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
