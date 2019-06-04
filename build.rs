use ructe::{Result, Ructe};
use std::{
	env,
	fs,
	path::PathBuf
};

fn main() -> Result<()> {
	let system_template_dir = format!("{}/templates-system", env::var("CARGO_MANIFEST_DIR").unwrap());
	let system_template_out = format!("{}/templates-system", env::var("OUT_DIR").unwrap());
	fs::create_dir_all(&system_template_dir).unwrap();
	fs::create_dir_all(&system_template_out).unwrap();
	let system_template_dir = PathBuf::from(system_template_dir);
	let system_template_out = PathBuf::from(system_template_out);

	let mut ructe_compiler = Ructe::new(system_template_out)?;
	ructe_compiler.compile_templates(&system_template_dir)
}
