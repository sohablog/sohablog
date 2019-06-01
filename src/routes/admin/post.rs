use rocket_codegen::*;
use rocket_contrib::{
	templates::Template
};
use rocket::{
	State,
	response::Redirect,
	request::LenientForm
};
use crate::{
	db::Database,
	render,
	models::{
		user::{self,User},
		content::{self,Content}
	},
};
use super::super::error::Error;

#[get("/admin/post")]
pub fn list(db: State<Database>,global_var: render::GlobalVariable,current_user: User)->Result<Template,Error>{
	current_user.check_permission(user::PERM_POST_VIEW)?;
	let mut ctx=tera::Context::new();
	let posts=Content::find_posts(&db,(0,25),true)?;
	ctx.insert("posts",&posts);
	Ok(render::render("admin/post/list",global_var,Some(ctx))?)
}

#[get("/admin/post/_new")]
pub fn new_get(global_var: render::GlobalVariable,current_user: User)->Result<Template,Error>{
	current_user.check_permission(user::PERM_POST_EDIT)?;
	Ok(render::render("admin/post/edit",global_var,None)?)
}
#[get("/admin/post/<post_id>")]
pub fn edit_get(post_id:i32,db: State<Database>,global_var: render::GlobalVariable,current_user: User)->Result<Template,Error>{
	current_user.check_permission(user::PERM_POST_EDIT)?;
	let post:Content=Content::find(&db,post_id)?;
	let mut ctx=tera::Context::new();
	if post.status==content::ContentStatus::Deleted || post.r#type!=content::ContentType::Article{
		return Err(Error::NotFound)
	}
	ctx.insert("post",&post);
	Ok(render::render("admin/post/edit",global_var,Some(ctx))?)
}
#[derive(Default,FromForm,Debug)]
pub struct PostForm{
	pub id: Option<i32>,
	pub title: Option<String>,
	pub content: String,
	pub slug: Option<String>,
	pub time: String
}
#[post("/admin/post/_edit",data="<form>")]
pub fn edit_post(db: State<Database>,form: LenientForm<PostForm>,current_user: User)->Result<Redirect,Error>{
	current_user.check_permission(user::PERM_POST_EDIT)?;
	let title=match &form.title{
		Some(title)=>if title.trim().len()==0{
			None
		}else{
			Some(title.to_owned())
		},
		None=>None
	};
	let slug=match &form.slug{
		Some(slug)=>if slug.trim().len()==0{
			None
		}else{
			Some(slug.to_owned())
		},
		None=>None
	};
	let _post=match form.id{
		Some(id)=>{
			let mut post=Content::find(&db,id)?;
			if post.status==content::ContentStatus::Deleted || post.r#type!=content::ContentType::Article{
				return Err(Error::NotFound)
			}
			post.title=title;
			post.slug=slug;
			post.content=form.content.to_owned();
			post.time=chrono::NaiveDateTime::parse_from_str(form.time.as_str(),"%Y-%m-%d %H:%M:%S")?;
			post.update(&db)?;
			post
		},
		None=>{
			// TODO: set view_password
			let content=content::NewContent{
				user: current_user.id,
				time: chrono::NaiveDateTime::parse_from_str(form.time.as_str(),"%Y-%m-%d %H:%M:%S")?,
				title: title,
				slug: slug,
				content: form.content.to_owned(),
				order_level: 0,
				r#type: content::ContentType::Article,
				status: content::ContentStatus::Normal,
				allow_comment: true,
				allow_feed: true,
				parent: None,
				view_password: None
			};
			Content::insert(&db,content)?
		}
	};
	Ok(Redirect::to("/admin/post"))
}
