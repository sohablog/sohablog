#![feature(proc_macro_hygiene,decl_macro,custom_attribute)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use] extern crate diesel;

mod db;
mod schema;
mod models;
mod routes;

fn main(){
	use rocket::routes;
	
	use crate::db::Database;
	use crate::routes as router;

    let db_url="mysql://dev_local:A1aB2bC3c@127.0.0.1:3306/sohablog";
	let mut db=Database::new(db_url);

	match db.init(){
		Ok(_)=>{
			rocket::ignite()
				.mount("/",routes![
					router::root::index,
					router::root::dev_test
				])
				.manage(db)
				.launch();
		},
		Err(e)=>println!("Met an error while initializing database: {}",e)
	};
}
