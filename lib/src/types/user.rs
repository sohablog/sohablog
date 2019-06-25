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
pub enum UserStatus {
	Normal = 0,
	Deleted = 1,
}
impl EnumType for UserStatus {
	fn try_from(n: i32) -> Result<Self> {
		match n {
			0 => Ok(Self::Normal),
			1 => Ok(Self::Deleted),
			_ => Err(Error::None),
		}
	}
	fn number(self) -> i32 { self as i32 }
}

#[cfg(feature = "main")]
sql_from_to!(UserStatus);
