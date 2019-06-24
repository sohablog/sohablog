pub trait DatabaseConnection {
	type Pool;
	fn pool(&self) -> &Self::Pool;
}
