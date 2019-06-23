use super::super::{error::Error, Page};
use crate::{
	models::{
		comment::{self, Comment},
		user::{self, User},
	},
	render::RenderResult,
	templates,
	util::*,
};
use rocket_codegen::*;

pub const ITEMS_PER_PAGE: i32 = 15;

#[get("/admin/comment?<page>&<status>")]
pub fn list(
	gctx: GlobalContext,
	page: Option<Page>,
	status: Option<comment::CommentStatus>,
	current_user: User,
) -> Result<RenderResult, Error> {
	current_user.check_permission(user::PERM_COMMENT_MANAGE)?;
	let mut page = page.unwrap_or_default();
	let status = status.unwrap_or(comment::CommentStatus::Normal);
	let comments =
		Comment::find_by_status(&gctx.db, page.range(ITEMS_PER_PAGE), &status)?;
	page.calc_total(
		Comment::count_by_status(&gctx.db, &status)? as i32,
		ITEMS_PER_PAGE,
	);

	Ok(render!(templates::admin::comment::list, &gctx, page, status, comments))
}
