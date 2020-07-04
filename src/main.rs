#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
#[macro_use] mod utils;
mod routes;
mod database;
mod schema;
mod github;

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

const HOME: &'static str = "/";
const BLOGS: &'static str = "/blogs";
const STATIC: &'static str = "/static";

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
    rocket::ignite()
        .attach(Template::custom(init_tera))
        .attach(PeriodicaidanDbConn::fairing())
        .mount(HOME, routes![index, about])
        .mount(BLOGS, routes![blog_index, get_blog])
        .mount(STATIC, StaticFiles::from(project_path!("/static")))
        .register(catchers![not_found])
        .launch();
}
