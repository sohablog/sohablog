#![feature(proc_macro_hygiene,decl_macro,custom_attribute,never_type,type_alias_enum_variants)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use] extern crate diesel;

mod db;
mod schema;
mod models;
mod routes;
mod render;

fn main(){
	use rocket::{
		self,
		routes
	};
	use rocket_contrib;
	
	use crate::db::Database;
	use crate::routes as router;

    let db_url="mysql://dev_local:A1aB2bC3c@127.0.0.1:3306/sohablog";
	let mut db=Database::new(db_url);

	match db.init(){
		Ok(_)=>{
			rocket::ignite()
				.mount("/",routes![
					router::root::index,
					router::user::login_get,
					router::user::login_post,
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
