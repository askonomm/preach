use rocket_dyn_templates::Template;

mod db;
mod models;
mod routes;
mod schema;
mod utils;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let routes = routes![
        routes::admin::index::index,
        routes::admin::setup::setup,
        routes::admin::setup::do_setup,
        routes::admin::login::login,
        routes::admin::login::do_login,
        routes::admin::posts::posts,
        routes::site::index::index,
        routes::site::blog::blog,
        routes::site::blog_post::blog_post,
    ];

    rocket::build()
        .mount("/", routes)
        .attach(Template::fairing())
}
