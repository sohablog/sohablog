use bcrypt;
use diesel;

pub enum Error{
	Database(diesel::result::Error),
	Bcrypt(bcrypt::BcryptError)
}
impl From<bcrypt::BcryptError> for Error{
	fn from(e: bcrypt::BcryptError)->Self{
		Error::Bcrypt(e)
	}
}
pub type Result<T>=std::result::Result<T,Error>;

pub mod user;
pub mod content;
