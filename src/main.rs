#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
#[macro_use] mod utils;
mod routes;
pub mod database;
mod schema;
mod github;
mod guards;

use routes::*;
use rocket_contrib::{
    database,
    templates::{Template, Engines},
    serve::StaticFiles,
    databases::diesel as rocket_diesel
};
use serde_json::Value;

#[database("periodicaidan")]
pub struct PeriodicaidanDbConn(rocket_diesel::PgConnection);

pub const HOME: &'static str = "/";
pub const BLOGS: &'static str = "/blogs";
pub const STATIC: &'static str = "/static";

fn init_tera(engines: &mut Engines) {
    engines.tera.register_function("static_url_for", Box::new(move |values| {
        let path = values.get("path")
            .and_then(Value::as_str);

        if let Some(p) = path {
            Ok(Value::from(format!("{}/{}", STATIC, p)))
        } else {
            Err("No path provided".into())
        }
    }));
}

fn main() {
    dotenv::dotenv().ok();

    rocket::ignite()
        .attach(Template::custom(init_tera))
        .attach(PeriodicaidanDbConn::fairing())
        .mount(HOME, routes![index, about])
        .mount(BLOGS, routes![blog_index, get_blog, new_blog, new_blog_form, new_blog_no_auth, get_tag])
        .mount(STATIC, StaticFiles::from(project_path!("/static")))
        .register(catchers![not_found])
        .launch();
}
