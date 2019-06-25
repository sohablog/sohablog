use rocket::{
	request::Request,
	response::{self, Responder},
};
use rocket_contrib::json::Json;
use serde_derive::*;
use std::string::ToString;

pub mod error;

#[derive(Debug, Serialize)]
pub struct ApiResult<T> {
	pub status: i32,
	pub r#return: String,
	pub data: T,
}
impl<T> ApiResult<T> {
	pub fn new(data: T, status: Option<i32>, rtn: Option<String>) -> Self {
		Self {
			status: status.unwrap_or(200),
			r#return: rtn.unwrap_or("OK".to_string()),
			data: data,
		}
	}
}

pub struct JsonOrNormal<J, N>(J, N);
impl<'r, J: serde::Serialize, N: Responder<'r>> Responder<'r> for JsonOrNormal<J, N> {
	fn respond_to(self, req: &Request) -> response::Result<'r> {
		if req
			.accept()
			.and_then(|o| o.first())
			.and_then(|o| Some(o.is_json()))
			.unwrap_or(false)
		{
			Json(self.0).respond_to(req)
		} else {
			self.1.respond_to(req)
		}
	}
}

pub use crate::utils::Page;

pub mod admin;
pub mod comment;
pub mod post;
pub mod root;
pub mod static_file;
pub mod user;
