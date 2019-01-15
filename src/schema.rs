table! {
    content (id) {
        id -> Integer,
        user -> Integer,
        created_at -> Datetime,
        modified_at -> Datetime,
        title -> Nullable<Varchar>,
        content -> Longtext,
        order_level -> Integer,
        #[sql_name = "type"]
        type_ -> Integer,
        status -> Integer,
        view_password -> Nullable<Varchar>,
        allow_comment -> Bool,
        allow_feed -> Bool,
        parent -> Nullable<Integer>,
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

joinable!(content -> user (user));

allow_tables_to_appear_in_same_query!(
    content,
    user,
);
