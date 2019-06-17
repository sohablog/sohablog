use super::super::error::Error;
use crate::{
	db::Database,
	models::{
		content::{self, Content},
		file::File,
		user::User,
	},
	util::*,
};
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

#[get("/admin/file/by-content/<content_id>")]
pub fn find_by_content(
	content_id: i32,
	db: State<Database>,
	_user: User,
) -> Result<Json<Vec<File>>, Error> {
	let content: Content = Content::find(&db, content_id)?;
	if content.status == content::ContentStatus::Deleted
		|| content.r#type != content::ContentType::Article
	{
		return Err(Error::NotFound);
	}
	let list = File::find_by_content_id(&db, content_id)?;
	Ok(Json(list))
}

// is CSRFTokenValidation needed?
#[delete("/admin/file/<id>")]
pub fn delete_by_id(
	id: i32,
	db: State<Database>,
	system_config: State<SystemConfig>,
	_user: User,
) -> Result<Status, Error> {
	let file: File = File::find(&db, id)?;
	match fs::remove_file(&file.key.replace("{upload_dir}", &system_config.upload_dir)) {
		Err(e) => match e.kind() {
			std::io::ErrorKind::NotFound => (),
			_ => return Err(Error::Io(e)),
		},
		_ => (),
	}
	file.delete(&db)?;
	Ok(Status::NoContent)
}

// is CSRFTokenValidation needed?
#[post("/admin/file/upload", data = "<data>")]
pub fn upload(
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
			let content_id = entries.fields.get("related_content_id").and_then(|o| {
				if let SavedData::Text(s) = &o[0].data {
					s.parse::<i32>().ok()
				} else {
					None
				}
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
			let file_key = format!(
				"{{upload_dir}}/{}/{}{}",
				&year_month,
				Uuid::new_v4(),
				&extension
			); // file key like `{upload_dir}/201906/mori.love`, this will be saved to db
			fs::create_dir_all(format!("{}/{}", &system_config.upload_dir, &year_month))?; // create folder like `{upload_dir}/201906`

			let save_path = file_key.replace("{upload_dir}", &system_config.upload_dir); // file full path for writing contents, like `/path/to/upload/201906/mori.love`
			match entries.fields.get("file")?[0].data {
				SavedData::Bytes(ref b) => {
					fs::write(&save_path, b)?;
				}
				SavedData::Text(ref s) => {
					fs::write(&save_path, s)?;
				}
				SavedData::File(ref path, _) => {
					fs::copy(path, &save_path)?;
				}
			}

			let file = File::create(
				&db,
				file_key,
				original_filename,
				current_user.id,
				content_id,
			)?;

			Ok(Json(file))
		}
		SaveResult::Partial(_, _) => Err(Error::HttpStatus(Status::NoContent)), // 204 No Content
		SaveResult::Error(e) => Err(Error::Io(e)),
	}
}
