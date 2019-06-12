use rocket_codegen::*;

use super::error::Error;
use super::{ApiResult, JsonOrNormal};
use crate::{
	models::{
		comment::{self, Comment, CommentStatus},
		content,
	},
	render::GlobalContext,
};
use rocket::{
	request::LenientForm,
	response::Redirect,
};

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
) -> Result<JsonOrNormal<ApiResult<Comment>, Redirect>, Error> {
	let content = content::Content::find(&gctx.db, content_id)?;
	if !content.user_has_access(gctx.user.as_ref()) {
		return Err(Error::NotFound);
	}

	let author = match &gctx.user {
		Some(u) => comment::Author::from_user(&u),
		None => {
			let name = data
				.name
				.as_ref()
				.ok_or(Error::BadRequest("Field `name` is required"))?
				.to_owned();
			let mail = data.mail.as_ref().and_then(|o| Some(o.to_owned()));
			let link = data.link.as_ref().and_then(|o| Some(o.to_owned()));
			if let None = mail {
				// TODO: Check if mail is required field
				return Err(Error::BadRequest("Field `mail` is required"));
			};
			// TODO: Check if link is required field
			comment::Author::new(name, mail, link)
		}
	};
	let mut parent: Option<i32> = None;
	let reply_to = if let Some(id) = data.reply_to {
		let reply_to_comment: Comment = Comment::find(&gctx.db, id)?;
		parent = Some(reply_to_comment.parent.unwrap_or(reply_to_comment.id));
		Some(reply_to_comment.id)
	} else {
		None
	};
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
		ApiResult::new(new_comment, None, None),
		Redirect::to("/"),
	))
}
