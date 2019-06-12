use crate::{models, render, util::*};
use rocket::{
	http::{Status},
	response::{self, Responder},
	Request,
};
use rocket_contrib::json::Json;
use super::ApiResult;

#[derive(Debug)]
pub enum Error {
	Model(models::Error),
	Render(render::Error),
	ChronoParse(chrono::ParseError),
	SerdeJson(serde_json::Error),
	NotFound,
	PermissionDenied,
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
			models::Error::UserHasNoPermission => Self::PermissionDenied,
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
impl From<serde_json::Error> for Error {
	fn from(err: serde_json::Error) -> Self {
		Self::SerdeJson(err)
	}
}
impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Self::Io(err)
	}
}

impl<'a> Responder<'a> for Error {
	fn respond_to(self, req: &Request) -> response::Result<'a> {
		let global_context = req.guard::<GlobalContext>().unwrap();

		println!("{:?}", &self);
		let status = match self {
			Self::HttpStatus(status) => status,
			Self::NotFound => Status::NotFound,
			Self::PermissionDenied => Status::Forbidden,
			Self::BadRequest(reason) => Status::new(400, reason),
			_ => Status::InternalServerError,
		};
		if req.accept().and_then(|o| o.first()).and_then(|o| Some(o.is_json())).unwrap_or(false) {
			Json(ApiResult {
				status: status.code.into(),
				r#return: if global_context.system_config.is_prod {
					status.reason.to_string()
				} else {
					format!("{:?}", &self)
				},
				data: ()
			}).respond_to(req)
		} else {
			Err(status)
		}
	}
}
