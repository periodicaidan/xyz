use rocket_contrib::templates::Template;
use crate::database::models::*;
use crate::{PeriodicaidanDbConn};
use rocket::Request;
use crate::github::GithubProject;
use rocket::response::{Redirect, Flash};
use crate::guards::{AuthToken, AuthError};
use rocket::request::Form;
use rocket::request::FlashMessage;
use rocket::http::Status;
use std::ops::Deref;
use crate::BLOGS;
use std::path::Path;
use serde::{Serialize, Deserialize};

/*** TOP-LEVEL ROUTES ***/

#[derive(Serialize, Deserialize)]
pub struct Social {
    pub name: String,
    pub link: String,
    pub icon: String,
}

impl Social {
    pub fn read_from_file(filepath: impl AsRef<Path>) -> Vec<Social> {
        std::fs::read(filepath.as_ref())
            .ok()
            .and_then(|bs| serde_json::from_slice::<Vec<Social>>(&bs).ok())
            .unwrap_or(vec![])
    }
}

#[get("/")]
pub fn index() -> Template {
    // let projects =
    //     GithubProject::read_from_file(project_path!("/static/data/github_projects.json"));
    let projects =
        std::fs::read_to_string(project_path!("/static/data/github_projects.json"))
            .unwrap();

    let socials =
        Social::read_from_file(project_path!("/static/data/socials.json"));

    render_template!("index", {
        "projects" => projects,
        "socials" => socials
    })
}

#[get("/about")]
pub fn about() -> Template {
    let content = std::fs::read_to_string(project_path!("/static/data/about.md"))
        .unwrap();
    render_template!("about", { "content" => content })
}

/*** BLOGS ***/

#[get("/")]
pub fn blog_index(conn: PeriodicaidanDbConn) -> Template {
    let blogs = Blog::all(&*conn);
    render_template!("blogs/index", {
        "blogs" => blogs
    })
}

#[derive(FromForm)]
pub struct BlogForm {
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: Option<String>,
    pub body: String,
    pub author: Option<String>,
}

#[post("/new", data = "<blog_form>")]
pub fn new_blog(conn: PeriodicaidanDbConn, blog_form: Option<Form<BlogForm>>, auth: AuthToken) -> Result<Redirect, Status> {
    if let Some(bf) = blog_form {
        let blog = create_blog(
            &*conn,
            &bf.title,
            if let Some(s) = &bf.subtitle { Some(&s) } else { None },
            if let Some(s) = &bf.author { Some(&s) } else { None },
            &bf.body,
            if let Some(s) = &bf.slug { Some(&s) } else { None }
        );
        Ok(Redirect::to(uri!(get_blog: blog.slug)))
    } else {
        Err(Status::BadRequest)
    }
}

#[get("/new")]
pub fn new_blog_form(flash: Option<FlashMessage>) -> Template {
    if let Some(f) = flash {
        render_template!("blogs/form", { "flash" => f.msg() })
    } else {
        render_template!("blogs/form")
    }
}

#[post("/new", data = "<blog_form>", rank = 2)]
pub fn new_blog_no_auth(blog_form: Option<Form<BlogForm>>) -> Flash<Redirect> {
    Flash::error(Redirect::to(uri!("/blogs", new_blog_form)), "Authorization failed.")
}

#[get("/<slug>", rank = 3)]
pub fn get_blog(conn: PeriodicaidanDbConn, slug: String) -> Template {
    let blog = Blog::by_slug(&*conn, &slug).unwrap();
    let tags = get_tags_for_blog(&*conn, &blog);
    render_template!("blogs/article", {
        "content" => blog,
        "tags" => tags
    })
}

#[get("/tags/<tag>")]
pub fn get_tag(conn: PeriodicaidanDbConn, tag: String) -> Template {
    let blogs = get_blogs_by_tag_name(&*conn, &tag);
    render_template!("blogs/tag", {
        "tag" => tag,
        "blogs" => blogs
    })
}

/*** ERRORS ***/

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    render_template!("errors/not_found", { "route" => req.uri().path() })
}