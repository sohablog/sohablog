#[cfg(not(feature = "lib-only"))]
use diesel::sql_types::*;
#[cfg(not(feature = "lib-only"))]
use serde_derive::*;

use super::*;

#[cfg_attr(not(feature = "lib-only"), derive(Serialize, Deserialize, FromSqlRow, AsExpression))]
#[cfg_attr(not(feature = "lib-only"), serde(rename_all = "lowercase"))]
#[cfg_attr(not(feature = "lib-only"), sql_type = "Integer")]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum CommentStatus {
	Normal = 0,
	Deleted = 1,
	Spam = 2,
	PendingReview = 3,
}
impl CommentStatus {
	// not impl std::convert::TryFrom　for some reasons
	pub fn try_from(n: i32) -> Result<Self> {
		match n {
			0 => Ok(CommentStatus::Normal),
			1 => Ok(CommentStatus::Deleted),
			2 => Ok(CommentStatus::Spam),
			3 => Ok(CommentStatus::PendingReview),
			_ => Err(Error::None),
		}
	}
}
