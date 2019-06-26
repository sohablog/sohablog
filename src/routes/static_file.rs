use crate::{
	templates::statics::StaticFile as SystemStaticFile,
	render::theme,
	utils::StaticFile,
	util::GlobalContext,
};
use mime::Mime;
use rocket::{
	http::{ContentType, Status},
	response::Content,
};
use rocket_codegen::*;

#[get("/system/<name>", rank = 12)]
pub fn system(name: String) -> Result<Content<&'static [u8]>, Status> {
	if let Some(f) = SystemStaticFile::get(name.as_str()) {
		let mime: &Mime = &f.mime;
		Ok(Content(
			ContentType::new(mime.type_().as_str(), mime.subtype().as_str()),
			f.content,
		))
	} else {
		Err(Status::NotFound)
	}
}

#[get("/theme/<name>", rank = 12)]
pub fn theme(gctx: GlobalContext, name: String) -> Result<Content<&'static [u8]>, Status> {
	if let Some(f) = theme::get_static(&gctx, &name) {
		let mime: &Mime = f.mime();
		Ok(Content(
			ContentType::new(mime.type_().as_str(), mime.subtype().as_str()),
			f.content(),
		))
	} else {
		Err(Status::NotFound)
	}
}
