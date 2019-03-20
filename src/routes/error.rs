use crate::{
	models,
	render
};
use rocket::{
	Request,
	response::{
		Responder,
		self
	},
	http::Status
};

#[derive(Debug)]
pub enum Error{
	Model(models::Error),
	Render(render::Error),
	ChronoParse(chrono::ParseError),
	NotFound,
	NoPermission
}

impl From<models::Error> for Error{
	fn from(err: models::Error)->Error{
		match err{
			models::Error::NotFound=>Error::NotFound,
			models::Error::UserHasNoPermission=>Error::NoPermission,
			_=>Error::Model(err)
		}
	}
}
impl From<render::Error> for Error{
	fn from(err: render::Error)->Error{
		Error::Render(err)
	}
}
impl From<chrono::ParseError> for Error{
	fn from(err: chrono::ParseError)->Error{
		Error::ChronoParse(err)
	}
}

impl<'a> Responder<'a> for Error{
	fn respond_to(self,_req: &Request)->response::Result<'a>{
		match self{
			Error::NotFound=>Err(rocket::http::Status::NotFound),
			Error::NoPermission=>Err(rocket::http::Status::Forbidden),
			_=>Err(Status::InternalServerError)
		}
	}
}
