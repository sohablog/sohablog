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
    comment (id) {
        id -> Integer,
        user -> Nullable<Integer>,
        author_name -> Varchar,
        author_mail -> Nullable<Varchar>,
        author_link -> Nullable<Varchar>,
        ip -> Nullable<Varchar>,
        user_agent -> Nullable<Varchar>,
        text -> Longtext,
        time -> Datetime,
        status -> Integer,
        parent -> Nullable<Integer>,
        content -> Integer,
        reply_to -> Nullable<Integer>,
    }
}

table! {
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
    file (id) {
        id -> Integer,
        key -> Varchar,
        filename -> Text,
        user -> Integer,
        content -> Nullable<Integer>,
        time -> Datetime,
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

joinable!(assoc_tag_content -> content (content));
joinable!(assoc_tag_content -> tag (tag));
joinable!(comment -> content (content));
joinable!(comment -> user (user));
joinable!(content -> category (category));
joinable!(content -> user (user));
joinable!(file -> content (content));
joinable!(file -> user (user));

allow_tables_to_appear_in_same_query!(
    assoc_tag_content,
    category,
    comment,
    content,
    file,
    tag,
    user,
);
