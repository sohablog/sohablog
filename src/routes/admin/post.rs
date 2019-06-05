use super::super::{error::Error, Page};
use crate::{
	db::Database,
	models::{
		self,
		content::{self, Content},
		user::{self, User},
	},
	render::{GlobalContext, RenderResult},
	templates
};
use rocket::{request::LenientForm, response::Redirect, State};
use rocket_codegen::*;

pub const ITEMS_PER_PAGE: i32 = 25;

#[get("/admin/post?<page>")]
pub fn list(
	gctx: GlobalContext,
	mut page: Page,
	current_user: User,
) -> Result<RenderResult, Error> {
	current_user.check_permission(user::PERM_POST_VIEW)?;
	let posts = content::Content::find_posts(&gctx.db, page.range(ITEMS_PER_PAGE), true)?;
	page.calc_total(
		content::Content::count_post(&gctx.db, false)? as i32,
		ITEMS_PER_PAGE,
	);

	Ok(render!(
		templates::admin::post::list,
		&gctx,
		page,
		posts
	))
}

#[get("/admin/post/_new")]
pub fn new_get(gctx: GlobalContext, current_user: User) -> Result<RenderResult, Error> {
	current_user.check_permission(user::PERM_POST_EDIT)?;
	let categories = models::category::Category::find_all(&gctx.db)?;

	Ok(render!(
		templates::admin::post::edit,
		&gctx,
		"New Post",
		None,
		categories
	))
}
#[get("/admin/post/<post_id>")]
pub fn edit_get(
	gctx: GlobalContext,
	post_id: i32,
	current_user: User,
) -> Result<RenderResult, Error> {
	current_user.check_permission(user::PERM_POST_EDIT)?;
	let post: Content = Content::find(&gctx.db, post_id)?;
	if post.status == content::ContentStatus::Deleted
		|| post.r#type != content::ContentType::Article
	{
		return Err(Error::NotFound);
	}
	let categories = models::category::Category::find_all(&gctx.db)?;
	Ok(render!(
		templates::admin::post::edit,
		&gctx,
		format!("Edit {}", post.title.as_ref().unwrap_or(&String::from("Untitled"))).as_str(),
		Some(post),
		categories
	))
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
