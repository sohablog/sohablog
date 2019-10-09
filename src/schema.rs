table! {
    assoc_tag_content (id) {
        id -> Int4,
        tag -> Int4,
        content -> Int4,
    }
}

table! {
    category (id) {
        id -> Int4,
        slug -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        order -> Int4,
        parent -> Nullable<Int4>,
    }
}

table! {
    comment (id) {
        id -> Int4,
        user -> Nullable<Int4>,
        author_name -> Varchar,
        author_mail -> Nullable<Varchar>,
        author_link -> Nullable<Varchar>,
        ip -> Nullable<Inet>,
        time -> Timestamptz,
        user_agent -> Nullable<Text>,
        text -> Text,
        status -> Int4,
        reply_to -> Nullable<Int4>,
        parent -> Nullable<Int4>,
        content -> Int4,
    }
}

table! {
    content (id) {
        id -> Int4,
        user -> Nullable<Int4>,
        created_at -> Timestamptz,
        modified_at -> Timestamptz,
        time -> Timestamptz,
        title -> Nullable<Varchar>,
        #[sql_name = "content"]
        content_ -> Text,
        draft_content -> Nullable<Text>,
        slug -> Nullable<Varchar>,
        category -> Nullable<Int4>,
        order_level -> Int4,
        #[sql_name = "type"]
        type_ -> Int4,
        status -> Int4,
        view_password -> Nullable<Varchar>,
        allow_comment -> Bool,
        allow_feed -> Bool,
        parent -> Nullable<Int4>,
    }
}

table! {
    file (id) {
        id -> Int4,
        key -> Varchar,
        filename -> Text,
        content -> Nullable<Int4>,
        user -> Int4,
        time -> Timestamptz,
    }
}

table! {
    tag (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    user (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        name -> Varchar,
        email -> Varchar,
        username_lower -> Varchar,
        email_lower -> Varchar,
        website -> Nullable<Text>,
        avatar_url -> Nullable<Text>,
        permission -> Int4,
        created_at -> Timestamptz,
        modified_at -> Timestamptz,
        last_login_time -> Timestamptz,
        status -> Int4,
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
