use rocket::{http::CookieJar, response::Redirect};

use crate::utils::is_authenticated;

#[get("/admin")]
pub fn index(cookies: &CookieJar<'_>) -> Redirect {
    if is_authenticated(cookies) {
        Redirect::to("/admin/posts")
    } else {
        Redirect::to("/admin/login")
    }
}
