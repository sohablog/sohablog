#![feature(proc_macro_hygiene, decl_macro)]

#[cfg(feature = "main")]
#[macro_use]
extern crate diesel;

pub mod interfaces;
pub mod utils;
#[macro_use]
pub mod plugin;
pub mod render;
pub mod types;
