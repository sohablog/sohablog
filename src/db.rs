use diesel::{mysql::MysqlConnection, r2d2::ConnectionManager};
use r2d2::Pool;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
	Database(diesel::result::Error),
	Pool(r2d2::Error),
}
impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::Database(_) => write!(f, "Database Error"),
			Error::Pool(_) => write!(f, "Pool Error"),
		}
	}
}
impl error::Error for Error {
	fn cause(&self) -> Option<&dyn error::Error> {
		match *self {
			Error::Database(ref e) => Some(e),
			Error::Pool(ref e) => Some(e),
		}
	}

	fn description(&self) -> &str {
		match *self {
			Error::Database(ref e) => e.description(),
			Error::Pool(ref e) => e.description(),
		}
	}
}
impl From<diesel::result::Error> for Error {
	fn from(e: diesel::result::Error) -> Error {
		Error::Database(e)
	}
}
impl From<r2d2::Error> for Error {
	fn from(e: r2d2::Error) -> Error {
		Error::Pool(e)
	}
}

pub use crate::utils::DatabaseConnection;

pub struct Database {
	pool: Option<Pool<ConnectionManager<MysqlConnection>>>,
	conn_url: String,
}
impl Database {
	pub fn new(conn_url: &str) -> Database {
		Database {
			pool: None,
			conn_url: conn_url.to_owned(),
		}
	}

	pub fn init(&mut self) -> Result<(), Error> {
		let manager = ConnectionManager::new(self.conn_url.to_owned());
		let pool = r2d2::Pool::builder().build(manager)?;
		self.pool = Some(pool);
		Ok(())
	}
}
impl DatabaseConnection for Database {
	type Pool = Pool<ConnectionManager<MysqlConnection>>;
	fn pool(&self) -> &Self::Pool {
		self.pool
			.as_ref()
			.expect("Database pool unavailable, forgot `init()`?")
	}
}
