use rocket_codegen::*;

use rocket::{
	State
};
use crate::db::Database;

#[get("/admin")]
pub fn index()->&'static str{
	"2333"
}
