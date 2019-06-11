use rocket_codegen::*;

use super::error::Error;
use crate::{
	db::Database,
	models::{content, user::User, comment::{self, Comment}}
};
use rocket_contrib::json::Json;
use rocket::{response::{self, Redirect}, http::Status, request::{LenientForm, State, Request}};

#[derive(Default, FromForm, Debug)]
pub struct NewCommentForm {
	pub name: Option<String>,
	pub mail: Option<String>,
	pub link: Option<String>,
	pub text: String,
	pub reply_to: Option<i32>,
}

#[post("/comment/content/<content_id>/xhr", data = "<data>")]
pub fn new_content_comment_xhr(content_id: i32, data: LenientForm<NewCommentForm>, db: State<Database>, current_user: Option<User>) -> Result<Status, Error> {
	let content = content::Content::find(&db, content_id)?;
	if !content.user_has_access(current_user.as_ref()) {
		return Err(Error::NotFound);
	}

	let author = match current_user {
		Some(u) => comment::Author::from_user(&u),
		None => {
			let name = data.name.as_ref().ok_or(Error::BadRequest("Field `name` is required"))?.to_owned();
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
	dbg!(&author);

	Ok(Status::NoContent)
}
