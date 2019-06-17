use rocket_codegen::*;

use super::error::Error;
use super::{ApiResult, JsonOrNormal};
use crate::{
	models::{
		comment::{self, Comment, CommentStatus, CommentSerializedNormal},
		content,
	},
	util::*,
};
use rocket::{
	http::{Cookie, Cookies},
	request::LenientForm,
	response::Redirect,
};
use validator;

#[derive(Default, FromForm, Debug)]
pub struct NewCommentForm {
	pub name: Option<String>,
	pub mail: Option<String>,
	pub link: Option<String>,
	pub text: String,
	pub reply_to: Option<i32>,
}

#[post("/comment/content/<content_id>", data = "<data>")]
pub fn new_content_comment(
	content_id: i32,
	data: LenientForm<NewCommentForm>,
	gctx: GlobalContext,
	_csrf: CSRFTokenValidation,
	mut cookies: Cookies,
) -> Result<JsonOrNormal<ApiResult<CommentSerializedNormal>, Redirect>, Error> {
	let content = content::Content::find(&gctx.db, content_id)?;
	if !content.user_has_access(gctx.user.as_ref()) {
		return Err(Error::NotFound);
	}

	if data.text.len() < 2 {
		return Err(Error::BadRequest("Reply content too short"));
	}

	let author = match &gctx.user {
		Some(u) => comment::Author::from_user(&u),
		None => {
			let name = data
				.name
				.as_ref()
				.and_then(|o| if o.trim().len() == 0 {
					None
				} else {
					Some(o.trim().to_string())
				})
				.ok_or(Error::BadRequest("Field `name` is required"))?
				.to_owned();
			let mail = data.mail.as_ref().and_then(|o| {
				let s = o.trim();
				if s.len() == 0 {
					None
				} else {
					Some(s.to_string())
				}
			});
			let link = data.link.as_ref().and_then(|o| {
				let s = o.trim();
				if s.len() == 0 || !validator::validate_url(s) {
					None
				} else {
					Some(s.to_string())
				}
			});
			if let Some(s) = &mail {
				if !validator::validate_email(s) {
					return Err(Error::BadRequest("Field `mail` is not valid"))
				}
			} else {
				// TODO: Check if mail is required field
				return Err(Error::BadRequest("Field `mail` is required"));
			}
			if let Some(s) = &link {
				if !validator::validate_url(s) {
					return Err(Error::BadRequest("Field `link` is not valid"))
				}
			} else {
				// TODO: Check if link is required field
				return Err(Error::BadRequest("Field `link` is required"));
			}
			comment::Author::new(name, mail, link)
		}
	};
	let mut parent: Option<i32> = None;
	let reply_to = if let Some(id) = data.reply_to {
		let reply_to_comment: Comment = Comment::find(&gctx.db, id)?;
		if reply_to_comment.content != content_id {
			return Err(Error::BadRequest("Invalid `reply_to`"));
		}
		parent = Some(reply_to_comment.parent.unwrap_or(reply_to_comment.id));
		Some(reply_to_comment.id)
	} else {
		None
	};

	cookies.add_private(Cookie::build("comment_author", serde_json::to_string(&author)?).path("/").permanent().finish());
	let new_comment = Comment::new(
		author,
		Some(gctx.ip.to_string()),
		gctx.user_agent.to_owned(),
		data.text.to_owned(),
		reply_to,
		parent,
		content_id,
		CommentStatus::Normal, // TODO: Default comment status setting
	);
	let new_comment = Comment::insert(&gctx.db, new_comment)?;

	Ok(JsonOrNormal(
		ApiResult::new(new_comment.serialize_normal(), None, None),
		Redirect::to(content.get_link()),
	))
}
