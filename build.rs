use ructe::{Result, Ructe};

fn main() -> Result<()> {
	Ructe::from_env()?.compile_templates(format!("{}/templates/admin", env!("CARGO_MANIFEST_DIR")))
}
