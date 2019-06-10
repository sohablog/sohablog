use crate::{
	templates::statics::StaticFile as SystemStaticFile,
	theme::templates::statics::StaticFile as ThemeStaticFile
};
use rocket::{
	http::{Status, ContentType},
	response::Content,
};
use rocket_codegen::*;
use mime::Mime;

#[get("/system/<name>", rank = 12)]
pub fn system(name: String) -> Result<Content<&'static [u8]>, Status> {
	if let Some(f) = SystemStaticFile::get(name.as_str()) {
		let mime: &Mime = &f.mime;
		Ok(Content(ContentType::new(mime.type_().as_str(), mime.subtype().as_str()), f.content))
	} else {
		Err(Status::NotFound)
	}
}

#[get("/theme/<name>", rank = 12)]
pub fn theme(name: String) -> Result<Content<&'static [u8]>, Status> {
	if let Some(f) = ThemeStaticFile::get(name.as_str()) {
		let mime: &Mime = &f.mime;
		Ok(Content(ContentType::new(mime.type_().as_str(), mime.subtype().as_str()), f.content))
	} else {
		Err(Status::NotFound)
	}
}
