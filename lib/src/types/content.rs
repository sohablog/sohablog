#[cfg(feature = "main")]
use diesel::sql_types::Integer;
#[cfg(feature = "main")]
use serde_derive::*;

use super::*;

#[cfg_attr(feature = "main", derive(Serialize, Deserialize, FromSqlRow, AsExpression))]
#[cfg_attr(feature = "main", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "main", sql_type = "Integer")]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum ContentType {
	Article = 0,
	SinglePage = 1,
}
impl EnumType for ContentType {
	fn try_from(n: i32) -> Result<Self> {
		match n {
			0 => Ok(Self::Article),
			1 => Ok(Self::SinglePage),
			_ => Err(Error::None),
		}
	}
	fn number(self) -> i32 { self as i32 }
}

#[cfg_attr(feature = "main", derive(Serialize, Deserialize, FromSqlRow, AsExpression))]
#[cfg_attr(feature = "main", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "main", sql_type = "Integer")]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum ContentStatus {
	Normal = 0,         // Shows in list, visible to public
	Deleted = 1,        // deleted
	Hidden = 2,         // Not shown in list, visible to public
	Unpublished = 3,    // only shows in admin panel, not visible everywhere
	WithAccessOnly = 4, // [not implemented] shows in list and visible only if logged in.
}
impl ContentStatus {
	pub const PUBLIC_LIST: [Self; 1] = [Self::Normal];
	pub const LOGGED_IN_LIST: [Self; 2] = [Self::Normal, Self::WithAccessOnly];
	pub const ADMIN_LIST: [Self; 4] = [
		Self::Normal,
		Self::Hidden,
		Self::Unpublished,
		Self::WithAccessOnly,
	];
	pub const PUBLIC_VISIBLE: [Self; 2] = [Self::Normal, Self::Hidden];
	pub const LOGGED_IN_VISIBLE: [Self; 3] = [Self::Normal, Self::Hidden, Self::WithAccessOnly];

	pub fn is_visible_to_public(&self) -> bool {
		Self::PUBLIC_VISIBLE.contains(self)
	}

	pub fn is_visible_to_logged_in(&self) -> bool {
		Self::LOGGED_IN_VISIBLE.contains(self)
	}
}
impl EnumType for ContentStatus {
	fn try_from(n: i32) -> Result<Self> {
		match n {
			0 => Ok(Self::Normal),
			1 => Ok(Self::Deleted),
			2 => Ok(Self::Hidden),
			3 => Ok(Self::Unpublished),
			4 => Ok(Self::WithAccessOnly),
			_ => Err(Error::None),
		}
	}
	fn number(self) -> i32 { self as i32 }
}

#[cfg(feature = "main")]
sql_from_to!(ContentStatus, ContentType);
