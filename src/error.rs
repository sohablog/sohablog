pub enum Kind {
	Database,
	DatabasePool,
}

pub trait Error {
	fn kind(&self) -> Kind;
}
