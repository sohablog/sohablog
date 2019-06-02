table! {
	category (slug) {
		slug -> Varchar,
		name -> Varchar,
		description -> Nullable<Text>,
		order -> Integer,
		parent -> Nullable<Varchar>,
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
        order_level -> Integer,
        #[sql_name = "type"]
        type_ -> Integer,
        status -> Integer,
        view_password -> Nullable<Varchar>,
        allow_comment -> Bool,
        allow_feed -> Bool,
        parent -> Nullable<Integer>,
        category -> Nullable<Varchar>,
    }
}

table! {
	use diesel::sql_types::*;	
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

allow_tables_to_appear_in_same_query!(
	category,
    content,
    user,
);
