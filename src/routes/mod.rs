use rocket::{
	http::{
		uri::{self, FromUriParam, Query, UriDisplay},
		RawStr,
	},
	request::FromFormValue,
};
use std::fmt::Result as FmtResult;

#[derive(Debug, Copy, Clone)]
pub struct Page {
	pub current: i32,
	pub total: i32,
}
impl Page {
	pub fn new(current: i32, total: i32) -> Self {
		Self {
			current: if current < 1 { 1 } else { current },
			total: total,
		}
	}

	pub fn calc_total(&mut self, item_count: i32, limit: i32) -> i32 {
		let mut t: i32 = item_count / limit;
		if item_count % limit != 0 {
			t += 1;
		}
		self.total = t;
		t
	}

	pub fn range(self, limit: i32) -> (i32, i32) {
		((self.current - 1) * limit, self.current * limit)
	}
}
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

pub mod error;

pub mod admin;
pub mod post;
pub mod root;
pub mod user;
pub mod static_file;
pub mod comment;
