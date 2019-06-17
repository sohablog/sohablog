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
mod util;
#[macro_use]
mod render;
mod routes;
mod schema;

fn main() {
	use crate::db::Database;
	use crate::routes as router;
	use crate::util::*;
	use rocket::{config::Config as RocketConfig, fairing::AdHoc, routes};
	use rocket_contrib::serve::StaticFiles;
	use std::env;

	dotenv::dotenv().ok();
	let rocket_config = RocketConfig::active().unwrap();

	let db_url = env::var("DATABASE_URL").unwrap();
	let mut db = Database::new(&db_url);
	let system_config = SystemConfig {
		upload_dir: env::var("SOHABLOG_UPLOAD_DIR").unwrap_or(String::from("upload/")),
		upload_route: env::var("SOHABLOG_UPLOAD_ROUTE").unwrap_or(String::from("/static/upload")),
		session_name: env::var("SOHABLOG_SESSION_NAME").unwrap_or(String::from("SOHABLOG_SESSION")),
		csrf_field_name: env::var("SOHABLOG_CSRF_FIELD_NAME").unwrap_or(String::from("_token")),
		real_ip_header: env::var("SOHABLOG_REAL_IP_HEADER").ok(),
		csrf_cookie_name: env::var("SOHABLOG_CSRF_COOKIE_NAME").ok(),
		is_prod: rocket_config.environment.is_prod(),
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
						router::comment::new_content_comment,
						router::admin::root::generate_password_hash,
						router::admin::root::index,
						router::admin::post::list,
						router::admin::post::new_get,
						router::admin::post::edit_get,
						router::admin::post::edit_post,
						router::admin::category::list,
						router::admin::category::update,
						router::admin::file::upload,
						router::admin::file::find_by_content,
						router::admin::file::delete_by_id
					],
				)
				.mount(
					system_config.upload_route.as_str(),
					StaticFiles::from(system_config.upload_dir.as_str()),
				)
				.mount(
					"/static/",
					routes!(router::static_file::system, router::static_file::theme),
				)
				.attach(AdHoc::on_response("General Info Header", |_, res| {
					res.set_raw_header(
						"X-Powered-By",
						concat!("SOHABlog/", env!("CARGO_PKG_VERSION")),
					);
				}))
				.attach(CSRFTokenValidation(None))
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
