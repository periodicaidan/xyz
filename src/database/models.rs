use chrono::{NaiveDateTime};
use diesel::PgConnection;
use diesel::prelude::*;
use regex::Regex;
use crate::schema::{blogs, tags, blogstags};
use chrono::format::StrftimeItems;
use diesel::associations::BelongsTo;
use std::iter::FromIterator;

#[derive(Queryable, Identifiable, serde::Serialize, Clone)]
#[table_name = "blogs"]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub subtitle: Option<String>,
    pub author: Option<String>,
    pub slug: String,
    pub body: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
}

lazy_static! {
    static ref DT_FMT: StrftimeItems<'static> = StrftimeItems::new("%B %-d, %Y");
}

pub struct BlogWithTags {
    blog: Blog,
    tags: Vec<Tag>
}

impl BlogWithTags {
    pub fn from_blog_and_tags(blog_tags: (&Blog, &Vec<Tag>)) -> Self {
        Self {
            blog: blog_tags.0.clone(),
            tags: blog_tags.1.clone()
        }
    }
}

impl Blog {
    pub fn all(conn: &PgConnection) -> Vec<Blog> {
        use crate::schema::blogs::dsl::*;
        let all_blogs: Vec<Blog> = blogs.filter(published.eq(true))
            .load::<Blog>(conn)
            .expect("Failed to load all blogs");

        all_blogs
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

    pub fn updated_at(&self) -> &NaiveDateTime {
        if let Some(dt) = &self.updated_at {
            &dt
        } else {
            &self.created_at
        }
    }

    pub fn formatted_created_at(&self) -> String {
        self.created_at.format_with_items(DT_FMT.clone()).to_string()
    }

    pub fn formatted_updated_at(&self) -> String {
        self.updated_at().format_with_items(DT_FMT.clone()).to_string()
    }
}

#[derive(Insertable)]
#[table_name="blogs"]
pub struct NewBlog<'b> {
    pub title: &'b str,
    pub subtitle: Option<&'b str>,
    pub author: Option<&'b str>,
    pub slug: &'b str,
    pub body: &'b str
}

pub fn create_blog<'b>(
    conn: &PgConnection,
    title: &'b str,
    subtitle: Option<&'b str>,
    author: Option<&'b str>,
    body: &'b str,
    slug: Option<&'b str>
) -> Blog {
    use crate::schema::blogs;

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
        author,
        slug: if let Some(s) = slug { s } else { &fallback_slug },
        body
    };

    add_blog(conn, &new_blog)
}

pub fn add_blog(conn: &PgConnection, blog: &NewBlog) -> Blog {
    use crate::schema::blogs;
    diesel::insert_into(blogs::table)
        .values(blog)
        .get_result(conn)
        .expect("Error saving new blog")
}

#[derive(Queryable, Identifiable, serde::Serialize, Clone)]
#[table_name = "tags"]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag<'t> {
    pub name: &'t str
}

#[derive(Queryable, Clone, Copy)]
#[derive(Identifiable)]
    #[table_name = "blogstags"]
    #[primary_key(blog_id, tag_id)]
#[derive(Associations)]
    #[belongs_to(Blog)]
    #[belongs_to(Tag)]
pub struct BlogTag {
    pub blog_id: i32,
    pub tag_id: i32,
}

pub fn create_tag(conn: &PgConnection, name: &str) -> Tag {
    let new_tag = NewTag { name };

    diesel::insert_into(tags::table)
        .values(&new_tag)
        .get_result(conn)
        .expect("Error saving new tag")
}

pub fn get_blogs_by_tag_name(conn: &PgConnection, tag_name: &str) -> Vec<Blog> {
    use diesel::pg::expression::array_comparison::any;
    let tid = {
        use crate::schema::tags::dsl::*;
        tags.select(id)
            .filter(name.eq(tag_name))
            .limit(1)
    };

    let blog_ids = {
        use crate::schema::blogstags::dsl::*;
        blogstags.select(blog_id)
            .filter(tag_id.eq(any(tid)))
    };

    use crate::schema::blogs::dsl::*;
    blogs.filter(id.eq(any(blog_ids)))
        .load::<Blog>(conn)
        .unwrap_or(vec![])
}

pub fn get_tags_for_blog(conn: &PgConnection, blog: &Blog) -> Vec<Tag> {
    let tag_ids = {
        use crate::schema::blogstags::dsl::*;
        BlogTag::belonging_to(blog)
            .select(tag_id)
    };

    use crate::schema::tags::dsl::*;
    use diesel::pg::expression::array_comparison::any;
    tags.filter(id.eq(any(tag_ids)))
        .load::<Tag>(conn)
        .ok()
        .unwrap_or(vec![])
}