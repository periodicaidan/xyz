table! {
    blogs (id) {
        id -> Int4,
        title -> Varchar,
        subtitle -> Nullable<Text>,
        author -> Nullable<Varchar>,
        slug -> Varchar,
        body -> Text,
        published -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    blogstags (blog_id, tag_id) {
        blog_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(blogstags -> blogs (blog_id));
joinable!(blogstags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    blogs,
    blogstags,
    tags,
);
