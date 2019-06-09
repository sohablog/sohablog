table! {
	assoc_tag_content (id) {
		id -> Integer,
		tag -> Integer,
		content -> Integer,
	}
}

table! {
    category (id) {
        id -> Integer,
        slug -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        order -> Integer,
        parent -> Nullable<Integer>,
    }
}

table! {
	use diesel::sql_types::*;
    content (id) {
        id -> Integer,
        user -> Integer,
        created_at -> Datetime,
        modified_at -> Datetime,
		time -> Datetime,
        title -> Nullable<Varchar>,
        slug -> Nullable<Varchar>,
		#[sql_name = "content"]
        content_ -> Longtext,
		draft_content -> Nullable<Longtext>,
        order_level -> Integer,
        #[sql_name = "type"]
        type_ -> Integer,
        status -> Integer,
        view_password -> Nullable<Varchar>,
        allow_comment -> Bool,
        allow_feed -> Bool,
        parent -> Nullable<Integer>,
        category -> Nullable<Integer>,
    }
}

table! {
	tag (id) {
		id -> Integer,
		name -> Varchar,
	}
}

table! {
    user (id) {
        id -> Integer,
        username -> Varchar,
        password_hash -> Varchar,
        name -> Varchar,
        email -> Varchar,
        username_lower -> Varchar,
        email_lower -> Varchar,
        avatar_url -> Nullable<Text>,
        permission -> Unsigned<Integer>,
        created_at -> Datetime,
        modified_at -> Datetime,
        last_login_time -> Datetime,
        status -> Integer,
    }
}

joinable!(content -> category (category));
joinable!(content -> user (user));
joinable!(assoc_tag_content -> content (content));
joinable!(assoc_tag_content -> tag (tag));

allow_tables_to_appear_in_same_query!(
    category,
    content,
    user,
	tag,
	assoc_tag_content,
);
