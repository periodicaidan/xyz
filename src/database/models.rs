use chrono::{NaiveDateTime};
use diesel::PgConnection;
use diesel::prelude::*;
use regex::Regex;
use crate::schema::blogs;

#[derive(Queryable, serde::Serialize, Clone)]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: String,
    pub body: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Blog {
    pub fn all(conn: &PgConnection) -> Vec<Self> {
        use crate::schema::blogs::dsl::*;
        blogs.filter(published.eq(true))
            .load::<Blog>(conn)
            .expect("Failed to load all blogs")
    }

    pub fn by_slug(conn: &PgConnection, target_slug: &str) -> Option<Self> {
        use crate::schema::blogs::dsl::*;
        blogs.filter(slug.eq(target_slug))
            .limit(1)
            .load::<Blog>(conn)
            .ok()
            .and_then(|bs|
                bs.get(0)
                    .map(Clone::clone)
            )
    }
}

#[derive(Insertable)]
#[table_name="blogs"]
pub struct NewBlog<'b> {
    pub title: &'b str,
    pub subtitle: Option<&'b str>,
    pub slug: &'b str,
    pub body: &'b str
}

pub fn create_blog<'b>(
    conn: &PgConnection,
    title: &'b str,
    subtitle: Option<&'b str>,
    body: &'b str,
    slug: Option<&'b str>
) -> Blog {
    use super::super::schema::blogs;

    fn make_slug_from_title(title: &str) -> String {
        lazy_static!{
            static ref RE: Regex = Regex::new(r#"[^A-Za-z0-9\-_]"#).unwrap();
        }

        let concat_title = title
            .to_ascii_lowercase()
            .replace(" ", "-");

        RE.replace_all(&concat_title, "").to_string()
    }

    let fallback_slug = make_slug_from_title(title);
    let new_blog = NewBlog {
        title,
        subtitle,
        slug: if let Some(s) = slug { s } else { &fallback_slug },
        body
    };

    diesel::insert_into(blogs::table)
        .values(&new_blog)
        .get_result(conn)
        .expect("Error saving new blog")
}