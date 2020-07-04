use rocket_contrib::templates::Template;
use crate::database::models::*;
use crate::PeriodicaidanDbConn;
use rocket::Request;
use crate::github::GithubProject;

/*** TOP-LEVEL ROUTES ***/

#[get("/")]
pub fn index() -> Template {
    let projects = map!{
        "meme" => "Mobile Enigma Machine Emulator (MEME)"
    };

    render_template!("index", {
        "projects" => GithubProject::get_plural(&projects)
    })
}

#[get("/about")]
pub fn about() -> Template {
    render_template!("about")
}

/*** BLOGS ***/

#[get("/")]
pub fn blog_index(conn: PeriodicaidanDbConn) -> Template {
    render_template!("blogs/index", { "blogs" => Blog::all(&*conn) })
}

#[get("/<slug>")]
pub fn get_blog(conn: PeriodicaidanDbConn, slug: String) -> Template {
    render_template!("blogs/article", { "content" => Blog::by_slug(&*conn, &slug) })
}

/*** ERRORS ***/

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    render_template!("errors/not_found", { "route" => req.uri().path() })
}