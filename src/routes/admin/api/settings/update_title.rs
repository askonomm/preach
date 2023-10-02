use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{form::Form, http::CookieJar};
use serde::Deserialize;

use crate::utils::data::is_authenticated;
use crate::utils::data::update_or_create_setting;

#[derive(FromForm)]
pub struct UpdateTitle<'a> {
    title: &'a str,
}

#[derive(Responder, Debug, Serialize, Deserialize)]
pub struct UpdateTitleResponse {
    status: &'static str,
    //message: String,
}

#[post("/admin/api/settings/update-title", data = "<form>")]
pub fn update_title(
    cookies: &CookieJar<'_>,
    form: Form<UpdateTitle<'_>>,
) -> Json<UpdateTitleResponse> {
    if !is_authenticated(cookies) {
        return Json(UpdateTitleResponse {
            status: "error",
            //message: "Not authenticated",
        });
    }

    update_or_create_setting("title", form.title);

    Json(UpdateTitleResponse {
        status: "success",
        //message: "Post updated",
    })
}
