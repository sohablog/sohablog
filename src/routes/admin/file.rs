use super::super::error::Error;
use crate::{
	models::user::User,
	db::Database,
};
use rocket_codegen::*;
use rocket::{
	http::{
		Status,
		ContentType
	},
	request::State,
	Data
};
use multipart::server::{
	Multipart,
	save::{
		SaveResult,
		SavedData,
		Entries
	}
};
use uuid::Uuid;

#[post("/admin/file/upload", data = "<data>")]
pub fn upload_file(data: Data, content_type: &ContentType, db: State<Database>, _user: User) -> Result<Status, Error> {
	if !content_type.is_form_data() {
		return Err(Error::BadRequest("Wrong `Content-Type`"));
	}
	let (_, boundary) = content_type.params().find(|&(k, _)| k == "boundary").ok_or_else(|| Error::BadRequest("No `boundary`"))?;

	match Multipart::with_body(data.open(), boundary).save().temp() {
		SaveResult::Full(entries) => {
			let filename = entries.fields.get("file")
				.and_then(|o| o.iter().next())
				.ok_or_else(|| Error::BadRequest("File not found in body"))?
				.headers.filename.clone();
			let extension = filename
				.and_then(|s| {
					s.rsplit('.').next()
						.and_then(|e| if e.chars().any(|c| !c.is_alphanumeric()) {
							None
						} else {
							Some(e.to_lowercase())
						})
				}).unwrap_or_default();
			let save_path = format!("static/upload/{}.{}", Uuid::new_v4(), extension);
			Ok(Status::NoContent)
		},
		SaveResult::Partial(_, _) => Ok(Status::Accepted),
		SaveResult::Error(e) => Err(Error::UploadError(e))
	}
}
