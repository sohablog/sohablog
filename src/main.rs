#![feature(
	proc_macro_hygiene,
	decl_macro,
	custom_attribute,
	never_type,
	vec_remove_item,
	try_trait
)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;

pub use sohablog_lib::{interfaces, plugin, types, utils};

mod db;
mod models;
#[macro_use]
mod render;
mod routes;
mod schema;
mod util;

fn main() {
	use crate::db::Database;
	use crate::routes as router;
	use crate::util::*;
	use rocket::{config::Config as RocketConfig, fairing::AdHoc, routes, http::Method};
	use rocket_contrib::serve::StaticFiles;
	use sohablog_lib::plugin::PluginManager;
	use std::env;

	dotenv::dotenv().ok();
	let rocket_config = RocketConfig::active().unwrap();
	let mut plugin_manager = PluginManager::new();

	let db_url = env::var("DATABASE_URL").unwrap();
	let mut db = Database::new(&db_url);
	let system_config = SystemConfig {
		plugin_dir: env::var("SOHABLOG_PLUGIN_DIR").unwrap_or(String::from("plugin/")),
		upload_dir: env::var("SOHABLOG_UPLOAD_DIR").unwrap_or(String::from("upload/")),
		upload_route: env::var("SOHABLOG_UPLOAD_ROUTE").unwrap_or(String::from("/static/upload")),
		session_name: env::var("SOHABLOG_SESSION_NAME").unwrap_or(String::from("SOHABLOG_SESSION")),
		robots_txt_path: env::var("SOHABLOG_ROBOTS_TXT_PATH").unwrap_or(String::from("robots.txt")),
		csrf_field_name: env::var("SOHABLOG_CSRF_FIELD_NAME").unwrap_or(String::from("_token")),
		real_ip_header: env::var("SOHABLOG_REAL_IP_HEADER").ok(),
		csrf_cookie_name: env::var("SOHABLOG_CSRF_COOKIE_NAME").ok(),
		is_prod: rocket_config.environment.is_prod(),
		theme_name: String::from("my-notebook"),
	};
	
	let robots_txt = util::RobotsTxt::new(get_robot_txt(&system_config.robots_txt_path));
	std::fs::create_dir_all(&system_config.upload_dir).unwrap();
	plugin_manager
		.load_from_dir(&system_config.plugin_dir)
		.unwrap();

	match db.init() {
		Ok(_) => {
			rocket::ignite()
				.mount("/", routes![
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
					router::admin::comment::list,
					router::admin::comment::set_status,
					router::admin::category::list,
					router::admin::category::update,
					router::admin::file::upload,
					router::admin::file::find_by_content,
					router::admin::file::delete_by_id
				])
				.mount("/", vec![rocket::Route::new(Method::Get, "/robots.txt", robots_txt)])
				.mount(
					&system_config.upload_route,
					StaticFiles::from(&system_config.upload_dir),
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
				.manage(Box::new(db))
				.manage(system_config)
				.manage(plugin_manager)
				.launch();
		}
		Err(e) => println!("Met an error while initializing database: {}", e),
	};
}

fn get_robot_txt(path: &str) -> Option<String> {
	use std::{path::Path, fs::read_to_string};
	let p = Path::new(path);

	if p.exists() && p.is_file() {
		read_to_string(p).map_err(|e| { dbg!(e); }).ok()
	} else {
		None
	}
}

// system templates
include!(concat!(env!("OUT_DIR"), "/templates-system/templates.rs"));

// user theme templates
mod theme {
	use sohablog_lib::utils::StaticFile;

	include!(concat!(env!("OUT_DIR"), "/templates-theme/templates.rs"));

	impl StaticFile for &templates::statics::StaticFile {
		fn content(&self) -> &'static [u8] {
			self.content
		}
		fn name(&self) -> &'static str {
			self.name
		}
		fn mime(&self) -> &'static mime::Mime {
			self.mime
		}
	}
}
