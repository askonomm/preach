use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket_dyn_templates::Template;

mod db;
mod models;
mod routes;
mod schema;
mod utils;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn run_migrations(connection: &mut impl MigrationHarness<diesel::pg::Pg>) {
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");
}

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    run_migrations(&mut db::connection());

    let routes = routes![
        routes::admin::index::index,
        routes::admin::setup::setup,
        routes::admin::setup::do_setup,
        routes::admin::login::login,
        routes::admin::login::do_login,
        routes::admin::logout::logout,
        routes::admin::posts::posts,
        routes::admin::posts::new_post,
        routes::admin::posts::edit_post,
        routes::admin::posts::delete_post,
        routes::admin::settings::settings,
        routes::admin::api::post::update_title::update_title,
        routes::admin::api::post::update_slug::update_slug,
        routes::admin::api::post::update_body::update_body,
        routes::admin::api::post::update_status::update_status,
        routes::admin::api::post::update_date::update_date,
        routes::admin::api::settings::update_title::update_title,
        routes::admin::api::settings::update_description::update_description,
        routes::admin::api::settings::update_short_description::update_short_description,
        routes::admin::api::settings::update_image::update_image,
        routes::admin::api::upload_image::upload_image,
        routes::uploads::uploads,
        routes::site::index::index,
        routes::site::blog::blog,
        routes::site::blog_post::blog_post,
    ];

    rocket::build()
        .mount("/", routes)
        .attach(Template::fairing())
}
