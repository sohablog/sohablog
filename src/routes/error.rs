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
	NotFound
}
impl From<models::Error> for Error{
	fn from(err: models::Error)->Error{
		match err{
			models::Error::NotFound=>Error::NotFound,
			_=>Error::Model(err)
		}
	}
}
impl From<render::Error> for Error{
	fn from(err: render::Error)->Error{
		Error::Render(err)
	}
}
impl<'a> Responder<'a> for Error{
	fn respond_to(self,_req: &Request)->response::Result<'a>{
		match self{
			Error::NotFound=>Err(rocket::http::Status::NotFound),
			_=>Err(Status::InternalServerError)
		}
	}
}
