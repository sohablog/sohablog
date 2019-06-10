use ructe::{Result, Ructe};
use std::{
	env,
	fs,
	path::PathBuf
};

fn compile_templates(src: String, statics: String, out: String) -> Result<()> {
	fs::create_dir_all(&src).unwrap();
	fs::create_dir_all(&statics).unwrap();
	fs::create_dir_all(&out).unwrap();
	let src = PathBuf::from(src);
	let statics = PathBuf::from(statics);
	let out = PathBuf::from(out);

	let mut ructe_compiler = Ructe::new(out)?;
	ructe_compiler.statics()?.add_files(&statics)?;
	ructe_compiler.compile_templates(&src)
}

fn main() -> Result<()> {
	compile_templates(
		format!("{}/templates-system", env::var("CARGO_MANIFEST_DIR").unwrap()),
		format!("{}/statics-system", env::var("CARGO_MANIFEST_DIR").unwrap()),
		format!("{}/templates-system", env::var("OUT_DIR").unwrap()),
	)?;
	compile_templates(
		format!("{}/templates-theme", env::var("CARGO_MANIFEST_DIR").unwrap()),
		format!("{}/statics-theme", env::var("CARGO_MANIFEST_DIR").unwrap()),
		format!("{}/templates-theme", env::var("OUT_DIR").unwrap()),
	)?;
	Ok(())
}
