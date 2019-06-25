#[cfg(not(feature = "lib-only"))]
use diesel::sql_types::*;
#[cfg(not(feature = "lib-only"))]
use serde_derive::*;

#[cfg_attr(not(feature = "lib-only"), derive(Serialize, Deserialize, FromSqlRow, AsExpression))]
#[cfg_attr(not(feature = "lib-only"), serde(rename_all = "lowercase"))]
#[cfg_attr(not(feature = "lib-only"), sql_type = "Integer")]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum UserStatus {
	Normal = 0,
	Deleted = 1,
}
