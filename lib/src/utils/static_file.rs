use mime::Mime;

pub trait StaticFile {
	fn content(&self) -> &'static [u8];
	fn name(&self) -> &'static str;
	fn mime(&self) -> &'static Mime;
}
