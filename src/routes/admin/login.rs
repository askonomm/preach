use rocket::{
    form::Form,
    http::{Cookie, CookieJar},
    response::Redirect,
};
use rocket_dyn_templates::{context, Template};
use uuid::Uuid;

use crate::utils::{authenticates, set_auth_token};

#[get("/admin/login")]
pub fn login() -> Template {
    Template::render(
        "admin/login",
        context! {
            email: ""
        },
    )
}

#[derive(FromForm)]
pub struct Login {
    email: String,
    password: String,
}

#[derive(Responder)]
pub enum LoginResponse {
    Template(Template),
    Redirect(Redirect),
}

#[post("/admin/login", data = "<login>")]
pub fn do_login(login: Form<Login>, cookies: &CookieJar<'_>) -> LoginResponse {
    if authenticates(&login.email, &login.password) {
        let auth_token = Uuid::new_v4().to_string();
        cookies.add(Cookie::new("auth_token", auth_token.clone()));
        set_auth_token(&login.email, &auth_token);

        println!("Logged in as {}", &login.email);

        return LoginResponse::Redirect(Redirect::to("/admin/posts"));
    }

    let context = context! {
        email: &login.email,
        error: "Invalid email or password",
    };

    LoginResponse::Template(Template::render("admin/login", &context))
}
