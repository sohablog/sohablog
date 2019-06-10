use super::super::error::Error;
use crate::{db::Database, models::{user::User, file::File}, SystemConfig};
use multipart::server::{
	save::{SaveResult, SavedData},
	Multipart,
};
use rocket::{
	http::{ContentType, Status},
	request::State,
	Data,
};
use rocket_codegen::*;
use rocket_contrib::json::Json;
use std::fs;
use uuid::Uuid;

#[post("/admin/file/upload", data = "<data>")]
pub fn upload_file(
	data: Data,
	content_type: &ContentType,
	system_config: State<SystemConfig>,
	db: State<Database>,
	current_user: User,
) -> Result<Json<File>, Error> {
	if !content_type.is_form_data() {
		return Err(Error::BadRequest("Wrong `Content-Type`"));
	}
	let (_, boundary) = content_type
		.params()
		.find(|&(k, _)| k == "boundary")
		.ok_or_else(|| Error::BadRequest("No `boundary`"))?;

	match Multipart::with_body(data.open(), boundary).save().temp() {
		SaveResult::Full(entries) => {
			let content_id = entries.fields.get("content")
				.and_then(|o| if let SavedData::Text(s) = &o[0].data {
					s.parse::<i32>().ok()
				} else {
					None
				});
				
			let filename = entries
				.fields
				.get("file")
				.and_then(|o| o.iter().next())
				.ok_or_else(|| Error::BadRequest("File not found in body"))?
				.headers
				.filename
				.clone();
			let original_filename = filename.as_ref()?.to_owned();
			let extension = filename
				.and_then(|s| {
					s.rsplit('.').next().and_then(|e| {
						if e.chars().any(|c| !c.is_alphanumeric()) {
							None
						} else {
							Some(format!(".{}", e.to_lowercase()))
						}
					})
				})
				.unwrap_or_default();

			let year_month = chrono::Utc::now().format("%Y%m");
			let file_key = format!("{{upload_dir}}/{}/{}{}", &year_month, Uuid::new_v4(), &extension); // file key like `{upload_dir}/201906/mori.love`, this will be saved to db
			fs::create_dir_all(format!("{}/{}", &system_config.upload_dir, &year_month))
				.map_err(|e| Error::UploadError(e))?; // create folder like `{upload_dir}/201906`

			let save_path = file_key.replace("{upload_dir}", &system_config.upload_dir); // file full path for writing contents, like `/path/to/upload/201906/mori.love`
			match entries.fields.get("file")?[0].data {
				SavedData::Bytes(ref b) => {
					fs::write(&save_path, b).map_err(|e| Error::UploadError(e))?;
				}
				SavedData::Text(ref s) => {
					fs::write(&save_path, s).map_err(|e| Error::UploadError(e))?;
				}
				SavedData::File(ref path, _) => {
					fs::copy(path, &save_path).map_err(|e| Error::UploadError(e))?;
				}
			}

			let file = File::create(&db, file_key, original_filename, current_user.id, content_id)?;

			Ok(Json(file))
		}
		SaveResult::Partial(_, _) => Err(Error::HttpStatus(Status::NoContent)), // 204 No Content
		SaveResult::Error(e) => Err(Error::UploadError(e)),
	}
}
