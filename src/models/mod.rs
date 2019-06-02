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
	OptionNone
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
	fn from(_: std::option::NoneError) -> Error {
		Error::OptionNone
	}
}
pub type Result<T> = std::result::Result<T, Error>;

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
			diesel::insert_into($table::table)
				.values(new)
				.execute(&*db.pool().get()?)?;
			Self::last(db)
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
macro_rules! insert_non_incremental {
	($table:ident, $from:ident, $pk_field:ident) => {
		pub fn insert(db: &crate::db::Database, new: $from) -> Result<Self> {
			diesel::insert_into($table::table)
				.values(&new)
				.execute(&*db.pool().get()?)?;
			Self::find(db, &new.$pk_field)
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
		pub fn update(&self,db: &crate::db::Database)->Result<()>{
			diesel::update(self).set(self).execute(&*db.pool().get()?)?;
			Ok(())
		}
	};
}
macro_rules! replace {
	()=>{
		pub fn replace(&self,db: &crate::db::Database,new: &Self)->Result<()>{
			diesel::update(self).set(new).execute(&*db.pool().get()?)?;
			Ok(())
		}
	};
}
/**
 * Get last row
 *
 * impl User{
 *     last!(table);
 * }
 * User::last(db);
 */
macro_rules! last {
	($table:ident) => { last!($table, id); };
	($table:ident, $field:ident) => {
		pub fn last(db: &crate::db::Database) -> Result<Self> {
			$table::table
				.order_by($table::$field.desc())
				.limit(1)
				.load::<Self>(&*db.pool().get()?)?
				.into_iter()
				.next()
				.ok_or(Error::NotFound)
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
	($table:ident) => { find_pk!($table, id as i32); };
	($table:ident, $field:ident as $field_type:ty) => {
		pub fn find(db: &crate::db::Database, pkv: $field_type) -> Result<Self> {
			$table::table
				.filter($table::$field.eq(pkv))
				.limit(1)
				.load::<Self>(&*db.pool().get()?)?
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
			$table::table$(.filter($table::$col.eq($col)))+.limit(1).load::<Self>(&*db.pool().get()?)?.into_iter().next().ok_or(Error::NotFound)
		}
	};
}

pub mod content;
pub mod user;
pub mod category;
