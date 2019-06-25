#[derive(Debug)]
pub enum Error {
	None,
}
pub type Result<T> = std::result::Result<T, Error>;

pub mod content;
pub use content::*;

pub mod user;
pub use user::*;

pub mod comment;
pub use comment::*;
