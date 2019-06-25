#![feature(
	proc_macro_hygiene,
	decl_macro,
	type_alias_enum_variants
)]

#[cfg(feature = "main")]
#[macro_use]
extern crate diesel;

pub mod interfaces;
pub mod utils;
pub mod plugin;
pub mod render;
pub mod types;
