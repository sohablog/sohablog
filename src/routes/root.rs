use rocket_codegen::*;

use rocket::{
	State
};
use crate::db::Database;

#[get("/")]
pub fn index()->&'static str{
	"2333"
}

#[get("/dev_testing")]
pub fn dev_test(db: State<Database>)->String{
	format!("{:?}",crate::models::user::User::find_by_username(&db,"soha").unwrap())
}
