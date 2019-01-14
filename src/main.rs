#![feature(proc_macro_hygiene,decl_macro)]

mod db;
mod routes;

use rocket::routes;

use crate::db::Database;
use crate::routes as router;

fn main(){
    let db_url="mysql://dev_local:A1aB2bC3c@127.0.0.1:3306/sohablog";
	let mut db=Database::new(db_url);

	match db.init(){
		Ok(_)=>{
			rocket::ignite()
				.mount("/",routes![
					router::root::index
				])
				.manage(db)
				.launch();
		}
		Err(e)=>println!("Met an error while initializing database: {}",e)
	};
}
