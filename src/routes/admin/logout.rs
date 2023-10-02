use rocket::{
    http::{Cookie, CookieJar},
    response::Redirect,
};

use crate::utils::data::{clear_auth_token, is_authenticated};

#[get("/admin/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    if !is_authenticated(cookies) {
        return Redirect::to("/admin/login");
    }

    clear_auth_token(cookies.get("auth_token").unwrap().value());
    cookies.remove(Cookie::named("auth_token"));

    Redirect::to("/admin/login")
}
