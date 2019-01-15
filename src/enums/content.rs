#[repr(u8)]
pub enum Status{
	Normal=0,
	Deleted=1
}

impl Status{
	fn from_num(n: u8)->Result<Status,String>{
		match n{
			0=>Ok(Status::Normal),
			1=>Ok(Status::Deleted),
			n=>Err(format!("Unknown kind: {}",n))
		}
	}
}
