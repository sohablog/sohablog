use crate::models;
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
	ModelError(models::Error)
}
impl From<models::Error> for Error{
	fn from(err: models::Error)->Error{
		Error::ModelError(err)
	}
}
impl<'a> Responder<'a> for Error{
	fn respond_to(self,_req: &Request)->response::Result<'a>{
		match self{
			Error::ModelError(e)=>match e{
				models::Error::NotFound=>Err(rocket::http::Status::NotFound),
				_=>Err(Status::InternalServerError)
			}
		}
	}
}
