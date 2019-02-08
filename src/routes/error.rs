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
	Render(render::Error)
}
impl From<models::Error> for Error{
	fn from(err: models::Error)->Error{
		Error::Model(err)
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
			Error::Model(e)=>match e{
				models::Error::NotFound=>Err(rocket::http::Status::NotFound),
				_=>Err(Status::InternalServerError)
			},
			_=>Err(Status::InternalServerError)
		}
	}
}
