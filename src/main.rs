#![feature(proc_macro_hygiene,decl_macro,custom_attribute,never_type,type_alias_enum_variants)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use] extern crate diesel;

mod db;
mod schema;
mod models;
mod routes;
mod render;

fn main(){
	use rocket::{routes};
	use std::env;
	use crate::db::Database;
	use crate::routes as router;

	dotenv::dotenv().ok();
	let db_url=env::var("DATABASE_URL").unwrap();
	let mut db=Database::new(&db_url);

	match db.init(){
		Ok(_)=>{
			rocket::ignite()
				.mount("/",routes![
					router::root::index,
					router::user::login_get,
					router::user::login_post,
					router::admin::root::generate_password_hash,
					router::admin::root::index,
					router::admin::post::list,
					router::admin::post::new_get,
					router::admin::post::edit_get,
					router::admin::post::edit_post,
					router::post::post_show,
					router::root::page_show
				])
				.attach(rocket_contrib::templates::Template::custom(|engines| {
					engines.tera.register_filter("markdown", render::tera_filter_markdown);
				}))
				.manage(db)
				.launch();
		},
		Err(e)=>println!("Met an error while initializing database: {}",e)
	};
}
