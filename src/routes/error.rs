use crate::{models, render};
use rocket::{
	http::Status,
	response::{self, Responder},
	Request,
};

#[derive(Debug)]
pub enum Error {
	Model(models::Error),
	Render(render::Error),
	ChronoParse(chrono::ParseError),
	NotFound,
	NoPermission,
	BadRequest(&'static str),
	UploadError(std::io::Error)
}

impl From<models::Error> for Error {
	fn from(err: models::Error) -> Error {
		match err {
			models::Error::NotFound => Error::NotFound,
			models::Error::UserHasNoPermission => Error::NoPermission,
			_ => Error::Model(err),
		}
	}
}
impl From<render::Error> for Error {
	fn from(err: render::Error) -> Error {
		Error::Render(err)
	}
}
impl From<chrono::ParseError> for Error {
	fn from(err: chrono::ParseError) -> Error {
		Error::ChronoParse(err)
	}
}

impl<'a> Responder<'a> for Error {
	fn respond_to(self, _req: &Request) -> response::Result<'a> {
		println!("{:?}", &self);
		match self {
			Error::NotFound => Err(rocket::http::Status::NotFound),
			Error::NoPermission => Err(rocket::http::Status::Forbidden),
			Error::BadRequest(reason) => Err(rocket::http::Status::new(400, reason)),
			_ => Err(Status::InternalServerError),
		}
	}
}
