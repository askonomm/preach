use rocket::{http::CookieJar, response::Redirect};

use crate::utils::data::{is_authenticated, is_setup};

#[get("/admin")]
pub fn index(cookies: &CookieJar<'_>) -> Redirect {
    if !is_setup() {
        return Redirect::to("/admin/setup");
    }

    if is_authenticated(cookies) {
        return Redirect::to("/admin/posts");
    }

    Redirect::to("/admin/login")
}
