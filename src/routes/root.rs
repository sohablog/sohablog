use rocket_codegen::*;

use crate::models;

#[get("/")]
pub fn index()->&'static str{
	"2333"
}

#[get("/dev_testing")]
pub fn dev_test()->String{
	match models::user::User::generate_password_hash("huaji"){
		Ok(s)=>s,
		Err(_e)=>"error!".to_string()
	}
}
