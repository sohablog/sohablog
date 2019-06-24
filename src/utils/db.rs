pub trait DatabaseConnection {
	type Connection;
	type Error;

	fn conn(&self) -> Result<Self::Connection, Self::Error>;
}
