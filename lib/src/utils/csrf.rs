use uuid::Uuid;
#[cfg(feature = "main")]
use serde_derive::*;

#[derive(Debug)]
#[cfg_attr(feature = "main", derive(Serialize, Deserialize))]
pub struct CSRFToken(String);
impl CSRFToken {
	pub fn validate(&self, s: &String) -> Result<(), ()> {
		if &self.0 == s {
			Ok(())
		} else {
			Err(())
		}
	}

	pub fn as_str(&self) -> &str {
		self.0.as_str()
	}
}
impl From<String> for CSRFToken {
	fn from(s: String) -> Self {
		Self(s)
	}
}
impl std::string::ToString for CSRFToken {
	fn to_string(&self) -> String {
		self.0.to_owned()
	}
}

impl From<Uuid> for CSRFToken {
	fn from(uuid: Uuid) -> Self {
		Self(uuid.to_simple().to_string())
	}
}
