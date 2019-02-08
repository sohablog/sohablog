use tera::*;
use std::result::Result;
use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket::{
	Request,
	request::{
		FromRequest,
		Outcome
	}
};
use pulldown_cmark::Parser;
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

pub fn tera_filter_markdown(value: tera::Value,_: HashMap<String,tera::Value>)->tera::Result<tera::Value>{
    let s=try_get_value!("markdown","value",String,value);
	let md_parser=Parser::new(s.as_str());
	let mut html=String::new();
	pulldown_cmark::html::push_html(&mut html,md_parser);
    Ok(to_value(html).unwrap())
}
