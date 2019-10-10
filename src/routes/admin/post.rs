use super::super::{error::Error, Page};
use crate::{
	db::Database,
	models::{
		self,
		content::{self, Content},
		user::{self, User},
		IntoInterface,
	},
	render::RenderResult,
	templates,
	types::EnumType,
	util::*,
};
use rocket::{request::LenientForm, response::Redirect, State};
use rocket_codegen::*;
use chrono::{NaiveDateTime, Local, offset::TimeZone};

pub const ITEMS_PER_PAGE: i32 = 25;

#[get("/admin/post?<page>")]
pub fn list(
	gctx: GlobalContext,
	mut page: Page,
	current_user: User,
) -> Result<RenderResult, Error> {
	current_user.check_permission(user::PERM_POST_VIEW)?;
	let content_status = content::ContentStatus::ADMIN_LIST.to_vec();
	let posts =
		content::Content::find_posts(&gctx.db, page.range(ITEMS_PER_PAGE), &content_status, true)?;
	page.calc_total(
		content::Content::count_post(&gctx.db, &content_status)? as i32,
		ITEMS_PER_PAGE,
	);

	Ok(render!(
		templates::admin::post::list,
		&gctx.get_template_context(),
		page,
		posts.into_interface(&gctx.db)
	))
}

#[get("/admin/post/_new")]
pub fn new_get(gctx: GlobalContext, current_user: User) -> Result<RenderResult, Error> {
	current_user.check_permission(user::PERM_POST_EDIT)?;
	let categories = models::category::Category::find_all(&gctx.db)?;

	Ok(render!(
		templates::admin::post::edit,
		&gctx.get_template_context(),
		"New Post",
		None,
		categories.into_interface(&gctx.db)
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
		&gctx.get_template_context(),
		format!(
			"Edit {}",
			post.title.as_ref().unwrap_or(&String::from("Untitled"))
		)
		.as_str(),
		Some(post.into_interface(&gctx.db)),
		categories.into_interface(&gctx.db)
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
	pub status: i32,
	pub save_draft: bool,
}
#[post("/admin/post/_edit", data = "<form>")]
pub fn edit_post(
	db: State<Box<Database>>,
	form: LenientForm<PostForm>,
	current_user: User,
	_csrf: CSRFTokenValidation,
) -> Result<Redirect, Error> {
	current_user.check_permission(user::PERM_POST_EDIT)?;
	let title = form
		.title
		.as_ref()
		.filter(|t| t.trim().len() > 0)
		.map(|t| t.trim().to_string());
	let slug = form
		.slug
		.as_ref()
		.filter(|t| t.trim().len() > 0)
		.map(|t| t.trim().to_string());
	let category = if let Some(id) = form.category {
		let cat: models::category::Category = models::category::Category::find(&db, id)?;
		Some(cat.id)
	} else {
		None
	};
	let parsed_time = Local.from_local_datetime(&NaiveDateTime::parse_from_str(form.time.as_str(), "%Y-%m-%d %H:%M:%S")?).unwrap().into();
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
			post.status = content::ContentStatus::try_from(form.status)?;
			if form.save_draft {
				post.draft_content = Some(form.content.to_owned());
			} else {
				post.content = form.content.to_owned();
				post.draft_content = None;
			}
			post.time = parsed_time;
			post.category = category;
			post.update(&db)?;
			post
		}
		None => {
			let ctxt = &form.content;
			// TODO: set view_password
			let content = content::NewContent {
				user: Some(current_user.id),
				time: parsed_time,
				title: title,
				slug: slug,
				content: if form.save_draft {
					String::from("This is an draft.")
				} else {
					ctxt.to_owned()
				},
				draft_content: if form.save_draft {
					Some(ctxt.to_owned())
				} else {
					None
				},
				order_level: 0,
				r#type: content::ContentType::Article,
				status: if form.save_draft {
					content::ContentStatus::Unpublished
				} else {
					content::ContentStatus::try_from(form.status)?
				},
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
