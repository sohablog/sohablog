#[derive(Debug)]
pub enum Error {
	None,
}
pub type Result<T> = std::result::Result<T, Error>;

pub trait EnumType: Sized {
	fn try_from(n: i32) -> Result<Self> where Self: Sized;
	fn number(self) -> i32;
}

#[cfg(feature = "main")]
macro_rules! sql_from_to {
	(@impl_one $enum:path) => {
		#[cfg(feature = "main")]
		impl<B> FromSql<diesel::sql_types::Integer, B> for $enum
		where
			B: Backend,
			i32: FromSql<diesel::sql_types::Integer, B>,
		{
			fn from_sql(bytes: Option<&B::RawValue>) -> deserialize::Result<Self> {
				let i = i32::from_sql(bytes)?;
				match Self::try_from(i) {
					Ok(s) => Ok(s),
					Err(_) => Err(format!("Failed convert enum value: `{}`", i).into()),
				}
			}
		}
		#[cfg(feature = "main")]
		impl<B> ToSql<diesel::sql_types::Integer, B> for $enum
		where
			B: Backend,
			i32: ToSql<diesel::sql_types::Integer, B>,
		{
			fn to_sql<W: std::io::Write>(
				&self,
				out: &mut serialize::Output<W, B>,
			) -> serialize::Result {
				(*self as i32).to_sql(out)
			}
		}
	};
	($($enum:path),+) => {
		#[cfg(feature = "main")]
		use diesel::{
			deserialize::{self, FromSql},
			serialize::{self, ToSql},
			backend::Backend,
		};
		$(sql_from_to!(@impl_one $enum);)+
	}
}

pub mod content;
pub use content::*;

pub mod user;
pub use user::*;

pub mod comment;
pub use comment::*;
