use diesel::QueryDsl;
use diesel::RunQueryDsl;
use rocket::form::Form;
use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};
use uuid::Uuid;

use crate::db;
use crate::models;
use crate::schema;

fn is_setup() -> bool {
    use self::schema::users::dsl::{id, users};

    let user_ids = users.select(id).load::<i32>(&mut db::connection());

    match user_ids {
        Ok(results) => !results.is_empty(),
        Err(_) => false,
    }
}

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

    // Create user
    let new_user = models::NewUser {
        email: &setup.email,
        password: &setup.password,
        auth_token: &auth_token,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    let user = diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(&mut db::connection());

    if user.is_err() {
        println!("Error creating user: {:?}", user)
    }

    // E woalaa
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
