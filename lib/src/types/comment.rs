#[cfg(feature = "main")]
use diesel::sql_types::Integer;
#[cfg(feature = "main")]
use serde_derive::*;

use super::*;
use crate::render::ToHtml;

#[cfg_attr(feature = "main", derive(Serialize, Deserialize, FromSqlRow, AsExpression))]
#[cfg_attr(feature = "main", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "main", sql_type = "Integer")]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum CommentStatus {
	Normal = 0,
	Deleted = 1,
	Spam = 2,
	PendingReview = 3,
}
impl EnumType for CommentStatus {
	fn try_from(n: i32) -> Result<Self> {
		match n {
			0 => Ok(Self::Normal),
			1 => Ok(Self::Deleted),
			2 => Ok(Self::Spam),
			3 => Ok(Self::PendingReview),
			_ => Err(Error::None),
		}
	}
	fn number(self) -> i32 { self as i32 }
}
impl ToHtml for CommentStatus {
	fn to_html(&self, out: &mut dyn std::io::Write) -> std::io::Result<()> {
		write!(out, "{}", *self as i32)
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
impl<'a> FromFormValue<'a> for CommentStatus {
	type Error = &'a RawStr;
	fn default() -> Option<Self> {
		Some(Self::Normal)
	}
	fn from_form_value(form_value: &'a RawStr) -> std::result::Result<Self, &'a RawStr> {
		match form_value.parse::<i32>() {
			Ok(status) => Ok(Self::try_from(status).map_err(|_| RawStr::from_str("No such CommentStatus"))?),
			_ => Err("Error when parsing `CommentStatus`".into()),
		}
	}
}
#[cfg(feature = "main")]
impl FromUriParam<Query, Option<CommentStatus>> for CommentStatus {
	type Target = CommentStatus;
	fn from_uri_param(v: Option<Self>) -> Self {
		v.unwrap_or(Self::Normal)
	}
}
#[cfg(feature = "main")]
impl UriDisplay<Query> for CommentStatus {
	fn fmt(&self, f: &mut uri::Formatter<Query>) -> std::fmt::Result {
		f.write_value(&format!("{}", *self as i32))
	}
}

#[cfg(feature = "main")]
sql_from_to!(CommentStatus);
