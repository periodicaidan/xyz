table! {
    blogs (id) {
        id -> Int4,
        title -> Varchar,
        subtitle -> Nullable<Text>,
        slug -> Varchar,
        body -> Text,
        published -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}
