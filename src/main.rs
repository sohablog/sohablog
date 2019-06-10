#![feature(
	proc_macro_hygiene,
	decl_macro,
	custom_attribute,
	never_type,
	type_alias_enum_variants,
	vec_remove_item,
	try_trait
)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;

mod db;
mod models;
#[macro_use]
mod render;
mod routes;
mod schema;

pub struct SystemConfig {
	pub upload_dir: String
}

fn main() {
	use crate::db::Database;
	use crate::routes as router;
	use rocket::routes;
	use std::env;
	use rocket_contrib::serve::StaticFiles;

	dotenv::dotenv().ok();
	let db_url = env::var("SOHABLOG_DATABASE_URL").unwrap();
	let mut db = Database::new(&db_url);
	let system_config = SystemConfig {
		upload_dir: env::var("SOHABLOG_UPLOAD_DIR").unwrap_or(String::from("upload/"))
	};
	std::fs::create_dir_all(system_config.upload_dir.as_str()).unwrap();

	match db.init() {
		Ok(_) => {
			rocket::ignite()
				.mount(
					"/",
					routes![
						router::root::index,
						router::post::post_show,
						router::root::page_show,
						router::user::login_get,
						router::user::login_post,
						router::admin::root::generate_password_hash,
						router::admin::root::index,
						router::admin::post::list,
						router::admin::post::new_get,
						router::admin::post::edit_get,
						router::admin::post::edit_post,
						router::admin::category::list,
						router::admin::category::update,
						router::admin::file::upload_file
					],
				)
				.mount("/static/upload", StaticFiles::from(system_config.upload_dir.as_str()))
				.manage(db)
				.manage(system_config)
				.launch();
		}
		Err(e) => println!("Met an error while initializing database: {}", e),
	};
}

// system templates
include!(concat!(env!("OUT_DIR"), "/templates-system/templates.rs"));

// user theme templates
mod theme {
	include!(concat!(env!("OUT_DIR"), "/templates-theme/templates.rs"));
}
