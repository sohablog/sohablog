use bcrypt;
use diesel;
use r2d2;

#[derive(Debug)]
pub enum Error {
	Database(diesel::result::Error),
	Bcrypt(bcrypt::BcryptError),
	Pool(r2d2::Error),
	NotFound,
	UserHasNoPermission,
	OptionNone,
	EnumType(crate::types::Error),
}
impl From<bcrypt::BcryptError> for Error {
	fn from(e: bcrypt::BcryptError) -> Self {
		Error::Bcrypt(e)
	}
}
impl From<diesel::result::Error> for Error {
	fn from(e: diesel::result::Error) -> Self {
		Error::Database(e)
	}
}
impl From<r2d2::Error> for Error {
	fn from(e: r2d2::Error) -> Self {
		Error::Pool(e)
	}
}
impl From<std::option::NoneError> for Error {
	fn from(_: std::option::NoneError) -> Self {
		Error::OptionNone
	}
}
impl From<crate::types::Error> for Error {
	fn from(e: crate::types::Error) -> Self {
		Self::EnumType(e)
	}
}
pub type Result<T> = std::result::Result<T, Error>;

pub struct RepositoryWrapper<T, D>(pub T, pub D);

pub trait IntoInterface<T>: Sized {
	fn into_interface(self, db: &Box<crate::db::Database>) -> T;
}
impl<T, E> IntoInterface<Option<T>> for Option<E>
where
	E: IntoInterface<T>,
{
	fn into_interface(self, db: &Box<crate::db::Database>) -> Option<T> {
		self.map(|c| c.into_interface(db))
	}
}
impl<T, E> IntoInterface<Vec<T>> for Vec<E>
where
	E: IntoInterface<T>,
{
	fn into_interface(self, db: &Box<crate::db::Database>) -> Vec<T> {
		self.into_iter()
			.map(|c| c.into_interface(db))
			.collect::<Vec<T>>()
	}
}
macro_rules! create_into_interface {
	($type:ty, $origin:ty) => {
		impl crate::models::IntoInterface<Box<$type>> for $origin {
			fn into_interface(self, db: &Box<Database>) -> Box<$type> {
				Box::new(RepositoryWrapper(self, db.clone())) as Box<$type>
			}
		}
	};
}

/**
 * Insert a new row
 *
 * impl User{
 *     insert!(table,NewUser);
 * }
 * User::insert(db,NewUser::new());
 */
macro_rules! insert {
	($table:ident, $from:ident) => {
		pub fn insert(db: &crate::db::Database, new: $from) -> Result<Self> {
			use crate::utils::DatabaseConnection;
			diesel::insert_into($table::table)
				.values(new)
				.get_result(&db.conn()?)
				.map_err(Error::from)
		}
	};
}
/**
 * Update a new row
 *
 * impl User{
 *     update!();
 * }
 * user.update(db);
 */
macro_rules! update {
	()=>{
		pub fn update(&self,db: &crate::db::Database) -> Result<()>{
			use crate::utils::DatabaseConnection;
			diesel::update(self).set(self).execute(&db.conn()?)?;
			Ok(())
		}
	};
}
/**
 * Delete row
 */
macro_rules! delete {
	() => {
		pub fn delete(&self,db: &crate::db::Database) -> Result<()> {
			use crate::utils::DatabaseConnection;
			diesel::delete(self)
				.execute(&db.conn()?)?;
			Ok(())
		}
	};
}
/**
 * Find row by id
 *
 * impl User{
 *     find_pk!(table);
 * }
 * User::find_pk(db,1);
 */
macro_rules! find_pk {
	($table:ident) => {
		find_pk!($table, id as i32);
	};
	($table:ident, $field:ident as $field_type:ty) => {
		pub fn find(db: &crate::db::Database, pkv: $field_type) -> Result<Self> {
			use crate::utils::DatabaseConnection;
			$table::table
				.filter($table::$field.eq(pkv))
				.limit(1)
				.load::<Self>(&db.conn()?)?
				.into_iter()
				.next()
				.ok_or(Error::NotFound)
		}
	};
}
/**
 * Find one by ...
 *
 * impl User{
 *     find_one_by!(user,find_by_username,username as &str);
 * }
 * User::find_by_username(db,"soha");
 */
macro_rules! find_one_by {
	($table:ident, $fn:ident, $($col:ident as $type:ty),+)=>{
		pub fn $fn(db: &crate::db::Database,$($col: $type),+)->Result<Self>{
			use crate::utils::DatabaseConnection;
			$table::table$(.filter($table::$col.eq($col)))+.limit(1).load::<Self>(&db.conn()?)?.into_iter().next().ok_or(Error::NotFound)
		}
	};
}
/**
 * Find by ...
 *
 * impl File{
 *     find_by!(file,find_by_content_id,content as i32);
 * }
 * File::find_by_content_id(db,1);
 */
macro_rules! find_by {
	($table:ident, $fn:ident, $($col:ident as $type:ty),+)=>{
		pub fn $fn(db: &crate::db::Database,$($col: $type),+)->Result<Vec<Self>>{
			use crate::utils::DatabaseConnection;
			$table::table$(.filter($table::$col.eq($col)))+.load::<Self>(&db.conn()?).map_err(Error::from)
		}
	};
}

pub mod category;
pub mod comment;
pub mod content;
pub mod file;
pub mod tag;
pub mod user;
