use rocket::{
	http::{
		uri::{self, FromUriParam, Query, UriDisplay},
		RawStr,
	},
	request::{FromFormValue, Request},
	response::{self, Responder},
};
use rocket_contrib::json::Json;
use serde_derive::*;
use std::{fmt::Result as FmtResult, string::ToString};

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
impl UriDisplay<Query> for Page {
	fn fmt(&self, f: &mut uri::Formatter<Query>) -> FmtResult {
		f.write_value(&format!("{}", self.current))
	}
}
impl FromUriParam<Query, Option<Page>> for Page {
	type Target = Page;
	fn from_uri_param(v: Option<Page>) -> Page {
		v.unwrap_or(Page::new(1, 1))
	}
}
impl<'a> FromFormValue<'a> for Page {
	type Error = &'a RawStr;
	fn default() -> Option<Self> {
		Some(Page::new(1, 1))
	}
	fn from_form_value(form_value: &'a RawStr) -> Result<Page, &'a RawStr> {
		match form_value.parse::<i32>() {
			Ok(page) => Ok(Page::new(page, 1)),
			_ => Err(form_value),
		}
	}
}

pub mod admin;
pub mod comment;
pub mod post;
pub mod root;
pub mod static_file;
pub mod user;
