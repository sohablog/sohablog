pub trait DatabaseConnection: Sync + Send {
	type Connection;
	type Error: std::error::Error;

	fn conn(&self) -> Result<Self::Connection, Self::Error>;
}
