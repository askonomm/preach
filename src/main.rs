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
        routes::admin::posts::posts,
        routes::site::index::index,
        routes::site::blog::blog,
        routes::site::blog_post::blog_post,
    ];

    rocket::build()
        .mount("/", routes)
        .attach(Template::fairing())
}
