use rocket_codegen::*;

use super::{error::Error, Page};
use crate::db::Database;
use crate::models::content;
use crate::render;
use rocket::State;
use rocket_contrib::templates::Template;

#[get("/?<page>")]
pub fn index(
	db: State<Database>,
	mut page: Page,
	global_var: render::GlobalVariable,
) -> Result<Template, Error> {
	let posts = content::Content::find_posts(&db, page.range(super::post::ITEMS_PER_PAGE), false)?;
	page.calc_total(
		content::Content::count_post(&db, false)? as i32,
		super::post::ITEMS_PER_PAGE,
	);
	
	let mut ctx = tera::Context::new();
	ctx.insert("posts", &posts);
	ctx.insert("pageTotal", &page.total);
	ctx.insert("pageCurrent", &page.current);
	Ok(render::theme_render("list", global_var, Some(ctx))?)
}

#[get("/<path>")]
pub fn page_show(
	db: State<Database>,
	global_var: render::GlobalVariable,
	path: String,
) -> Result<Template, Error> {
	let slug = path.replace(".html", ""); // TODO: We just need to remove `.html` at the end
	let post: content::Content = content::Content::find_by_slug(&db, &slug)?;
	if post.status == content::ContentStatus::Deleted
		|| post.r#type != content::ContentType::SinglePage
	{
		return Err(Error::NotFound);
	}
	// TODO: Password check when `view_password` exists
	let poster = post.get_user(&db)?;
	let mut ctx = tera::Context::new();
	ctx.insert("post", &post);
	ctx.insert("poster", &poster);
	Ok(render::theme_render("post", global_var, Some(ctx))?)
}
