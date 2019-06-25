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
impl Default for Page {
	fn default() -> Self {
		Page::new(1, 1)
	}
}

#[cfg(feature = "main")]
use rocket::{
	http::{
		uri::{self, FromUriParam, Query, UriDisplay},
		RawStr,
	},
	request::FromFormValue,
};
#[cfg(feature = "main")]
impl UriDisplay<Query> for Page {
	fn fmt(&self, f: &mut uri::Formatter<Query>) -> std::fmt::Result {
		f.write_value(&format!("{}", self.current))
	}
}
#[cfg(feature = "main")]
impl FromUriParam<Query, Option<Page>> for Page {
	type Target = Page;
	fn from_uri_param(v: Option<Page>) -> Page {
		v.unwrap_or(Page::new(1, 1))
	}
}
#[cfg(feature = "main")]
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
