use tera::Context;
use rocket_contrib::templates::Template;
use rocket::{
	Request,
	request::{
		FromRequest,
		Outcome
	}
};
use crate::models::user;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error{
	TemplateRender
}

pub fn theme_render(name: &str,global_var: GlobalVariable,context: Option<Context>)->Result<Template,Error>{
	let mut ctx=Context::new();
	ctx.insert("currentUser",&global_var.current_user);
	if let Some(context) = context {
		ctx.extend(context);
	};
	Ok(Template::render(format!("theme/{}/{}","basic",name), &ctx))
}

pub struct GlobalVariable{
	pub current_user: Option<user::User>
}
impl<'a,'r> FromRequest<'a,'r> for GlobalVariable{
	type Error=();
	fn from_request(request: &'a Request<'r>)->Outcome<GlobalVariable,()>{
		let user=request.guard::<Option<user::User>>().unwrap();
		Outcome::Success(GlobalVariable{
			current_user: user
		})
	}
}
