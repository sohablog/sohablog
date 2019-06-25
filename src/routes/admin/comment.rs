use super::super::{error::Error, Page, ApiResult, JsonOrNormal};
use crate::{
	models::{
		comment::{Comment, CommentStatus},
		user::{self, User},
		IntoInterface,
	},
	render::RenderResult,
	templates,
	util::*,
};
use rocket::response::Redirect;
use rocket_codegen::*;

pub const ITEMS_PER_PAGE: i32 = 15;

#[get("/admin/comment?<page>&<status>")]
pub fn list(
	gctx: GlobalContext,
	page: Option<Page>,
	status: Option<CommentStatus>,
	current_user: User,
) -> Result<RenderResult, Error> {
	current_user.check_permission(user::PERM_COMMENT_MANAGE)?;
	let mut page = page.unwrap_or_default();
	let status = status.unwrap_or(CommentStatus::Normal);
	let comments =
		Comment::find_by_status(&gctx.db, page.range(ITEMS_PER_PAGE), &status)?;
	page.calc_total(
		Comment::count_by_status(&gctx.db, &status)? as i32,
		ITEMS_PER_PAGE,
	);

	Ok(render!(templates::admin::comment::list, &gctx.get_template_context(), page, status, comments.into_interface(&gctx.db)))
}


#[post("/admin/comment/<id>/status/<status>")]
pub fn set_status(
	gctx: GlobalContext,
	id: i32,
	status: i32,
	current_user: User,
	_csrf: CSRFTokenValidation,
) -> Result<JsonOrNormal<ApiResult<()>, Redirect>, Error> {
	current_user.check_permission(user::PERM_COMMENT_MANAGE)?;
	let status = CommentStatus::try_from(status)?;
	let mut comment: Comment = Comment::find(&gctx.db, id)?;
	comment.status = status;
	comment.update(&gctx.db)?;
	Ok(JsonOrNormal(ApiResult::new((), None, None), Redirect::to(uri!(list: page = Some(Page::new(1, 1)), status = Some(comment.status)))))
}
