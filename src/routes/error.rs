use crate::{models, render};
use rocket::{
	http::Status,
	response::{self, Responder, Response},
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
	Io(std::io::Error),
	OptionNone,
	HttpStatus(Status),
}
impl From<std::option::NoneError> for Error {
	fn from(_: std::option::NoneError) -> Self {
		Self::OptionNone
	}
}
impl From<models::Error> for Error {
	fn from(err: models::Error) -> Self {
		match err {
			models::Error::NotFound => Self::NotFound,
			models::Error::UserHasNoPermission => Self::NoPermission,
			_ => Self::Model(err),
		}
	}
}
impl From<render::Error> for Error {
	fn from(err: render::Error) -> Self {
		Self::Render(err)
	}
}
impl From<chrono::ParseError> for Error {
	fn from(err: chrono::ParseError) -> Self {
		Self::ChronoParse(err)
	}
}
impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Self::Io(err)
	}
}

impl<'a> Responder<'a> for Error {
	fn respond_to(self, _req: &Request) -> response::Result<'a> {
		println!("{:?}", &self);
		match self {
			Self::HttpStatus(status) => {
				let mut resp = Response::new();
				resp.set_status(status);
				Ok(resp)
			},
			Self::NotFound => Err(Status::NotFound),
			Self::NoPermission => Err(Status::Forbidden),
			Self::BadRequest(reason) => Err(Status::new(400, reason)),
			_ => Err(Status::InternalServerError),
		}
	}
}
