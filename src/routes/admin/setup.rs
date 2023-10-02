use std::time::SystemTime;

use bcrypt::hash;
use bcrypt::DEFAULT_COST;
use diesel::prelude::*;
use rocket::form::Form;
use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};
use uuid::Uuid;

use crate::db;
use crate::models;
use crate::schema;
use crate::utils::data::is_setup;

#[derive(Responder)]
pub enum SetupResponse {
    Redirect(Redirect),
    Template(Template),
}

#[get("/admin/setup")]
pub fn setup() -> SetupResponse {
    if is_setup() {
        return SetupResponse::Redirect(Redirect::to("/admin"));
    }

    SetupResponse::Template(Template::render(
        "admin/setup",
        context! {
            email: "",
        },
    ))
}

#[derive(FromForm)]
pub struct Setup {
    email: String,
    password: String,
}

#[post("/admin/setup", data = "<setup>")]
pub fn do_setup(setup: Form<Setup>, cookies: &CookieJar<'_>) -> SetupResponse {
    if is_setup() {
        return SetupResponse::Redirect(Redirect::to("/admin"));
    }

    // Create auth token
    let auth_token = Uuid::new_v4().to_string();

    // Hash password
    let password = hash(setup.password.clone(), DEFAULT_COST).unwrap();

    // Create user
    let new_user = models::NewUser {
        email: &setup.email,
        password: &password,
        auth_token: &auth_token,
        created_at: SystemTime::now(),
        updated_at: SystemTime::now(),
    };

    let user = diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(&mut db::connection());

    // Log the user in or show an error
    match user {
        Ok(_) => {
            cookies.add(Cookie::new("auth_token", auth_token));
            SetupResponse::Redirect(Redirect::to("/admin"))
        }
        Err(_) => SetupResponse::Template(Template::render(
            "admin/setup",
            context! {
                email: setup.email.clone(),
                error: "Error creating user",
            },
        )),
    }
}
