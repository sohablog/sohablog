use super::super::error::Error;
use crate::{
	models::user::User,
	db::Database,
	SystemConfig
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
		SavedData
	}
};
use std::fs;
use uuid::Uuid;

#[post("/admin/file/upload", data = "<data>")]
pub fn upload_file(data: Data, content_type: &ContentType, system_config: State<SystemConfig>) -> Result<Status, Error> {
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
							Some(format!(".{}", e.to_lowercase()))
						})
				}).unwrap_or_default();
			// {UPLOAD_DIR}/yyyymm/
			let save_path = format!("{}/{}{}", &system_config.upload_dir, Uuid::new_v4(), extension);
			match entries.fields.get("file")?[0].data {
				SavedData::Bytes(ref b) => { fs::write(&save_path, b).map_err(|e| Error::UploadError(e))?; },
				SavedData::File(ref path, _) => { fs::copy(path, &save_path).map_err(|e| Error::UploadError(e))?; },
				_ => return Err(Error::BadRequest("SavedData is not accepted"))
			}

			// save some informations into db

			Ok(Status::NoContent) // 204 No Content
		},
		SaveResult::Partial(_, _) => Ok(Status::Accepted), // 202 Accepted
		SaveResult::Error(e) => Err(Error::UploadError(e))
	}
}
