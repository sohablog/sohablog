use super::super::{error::Error, Page};
use crate::{
	db::Database,
	models::{
		self,
		content::{self, Content},
		user::{self, User},
	},
	render,
};
use rocket::{request::LenientForm, response::Redirect, State};
use rocket_codegen::*;
use rocket_contrib::templates::Template;

pub const ITEMS_PER_PAGE: i32 = 25;

#[get("/admin/post?<page>")]
pub fn list(
	db: State<Database>,
	page: Option<Page>,
	global_var: render::GlobalVariable,
	current_user: User,
) -> Result<Template, Error> {
	current_user.check_permission(user::PERM_POST_VIEW)?;
	let page = page.unwrap_or_default();
	let posts = content::Content::find_posts(&db, page.range(ITEMS_PER_PAGE), true)?;
	let page_total = Page::total(
		content::Content::count_post(&db, false)? as i32,
		ITEMS_PER_PAGE,
	);

	let mut ctx = tera::Context::new();
	ctx.insert("posts", &posts);
	ctx.insert("pageTotal", &page_total);
	ctx.insert("pageCurrent", &page.0);
	Ok(render::render("admin/post/list", global_var, Some(ctx))?)
}

#[get("/admin/post/_new")]
pub fn new_get(
	db: State<Database>,
	global_var: render::GlobalVariable,
	current_user: User,
) -> Result<Template, Error> {
	current_user.check_permission(user::PERM_POST_EDIT)?;
	let categories = models::category::Category::find_all(&db)?;

	let mut ctx = tera::Context::new();
	ctx.insert("categories", &categories);
	Ok(render::render("admin/post/edit", global_var, None)?)
}
#[get("/admin/post/<post_id>")]
pub fn edit_get(
	post_id: i32,
	db: State<Database>,
	global_var: render::GlobalVariable,
	current_user: User,
) -> Result<Template, Error> {
	current_user.check_permission(user::PERM_POST_EDIT)?;
	let post: Content = Content::find(&db, post_id)?;
	let mut ctx = tera::Context::new();
	if post.status == content::ContentStatus::Deleted
		|| post.r#type != content::ContentType::Article
	{
		return Err(Error::NotFound);
	}
	let categories = models::category::Category::find_all(&db)?;
	let tags = post.get_tags(&db)?;
	let tags = tags.iter().map(|t| t.name.as_str()).collect::<Vec<&str>>();
	ctx.insert("post", &post);
	ctx.insert("categories", &categories);
	ctx.insert("tags", &tags);
	Ok(render::render("admin/post/edit", global_var, Some(ctx))?)
}
#[derive(Default, FromForm, Debug)]
pub struct PostForm {
	pub id: Option<i32>,
	pub title: Option<String>,
	pub content: String,
	pub slug: Option<String>,
	pub time: String,
	pub category: Option<i32>,
	pub tags: Option<String>,
}
#[post("/admin/post/_edit", data = "<form>")]
pub fn edit_post(
	db: State<Database>,
	form: LenientForm<PostForm>,
	current_user: User,
) -> Result<Redirect, Error> {
	current_user.check_permission(user::PERM_POST_EDIT)?;
	let title = match &form.title {
		Some(title) => {
			if title.trim().len() == 0 {
				None
			} else {
				Some(title.to_owned())
			}
		}
		None => None,
	};
	let slug = match &form.slug {
		Some(slug) => {
			if slug.trim().len() == 0 {
				None
			} else {
				Some(slug.to_owned())
			}
		}
		None => None,
	};
	let category = match form.category {
		Some(cat_id) => {
			let cat: models::category::Category = models::category::Category::find(&db, cat_id)?;
			Some(cat.id)
		}
		None => None,
	};
	let post = match form.id {
		Some(id) => {
			let mut post: Content = Content::find(&db, id)?;
			if post.status == content::ContentStatus::Deleted
				|| post.r#type != content::ContentType::Article
			{
				return Err(Error::NotFound);
			}
			post.title = title;
			post.slug = slug;
			post.content = form.content.to_owned();
			post.time =
				chrono::NaiveDateTime::parse_from_str(form.time.as_str(), "%Y-%m-%d %H:%M:%S")?;
			post.category = category;
			post.update(&db)?;
			post
		}
		None => {
			// TODO: set view_password
			let content = content::NewContent {
				user: current_user.id,
				time: chrono::NaiveDateTime::parse_from_str(
					form.time.as_str(),
					"%Y-%m-%d %H:%M:%S",
				)?,
				title: title,
				slug: slug,
				content: form.content.to_owned(),
				order_level: 0,
				r#type: content::ContentType::Article,
				status: content::ContentStatus::Normal,
				allow_comment: true,
				allow_feed: true,
				parent: None,
				view_password: None,
				category: category,
			};
			Content::insert(&db, content)?
		}
	};
	if let Some(tags) = &form.tags {
		let tags: Vec<&str> = tags.split(",").map(|s| s.trim()).collect();
		post.set_tags(&db, tags)?;
	}
	Ok(Redirect::to("/admin/post"))
}
