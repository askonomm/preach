use rocket::{http::CookieJar, response::Redirect};
use rocket_dyn_templates::{context, Template};

use crate::utils::data::{get_setting, is_authenticated};

#[derive(Responder)]
pub enum SettingsResponse {
    Redirect(Redirect),
    Template(Template),
}

#[get("/admin/settings")]
pub fn settings(cookies: &CookieJar<'_>) -> SettingsResponse {
    if !is_authenticated(cookies) {
        return SettingsResponse::Redirect(Redirect::to("/admin/login"));
    }

    SettingsResponse::Template(Template::render(
        "admin/settings",
        context! {
            image: get_setting("image"),
            title: get_setting("title"),
            description: get_setting("description"),
            short_description: get_setting("short_description"),
        },
    ))
}
