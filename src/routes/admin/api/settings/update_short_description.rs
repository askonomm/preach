use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{form::Form, http::CookieJar};
use serde::Deserialize;

use crate::utils::data::is_authenticated;
use crate::utils::data::update_or_create_setting;

#[derive(FromForm)]
pub struct UpdateShortDescription<'a> {
    description: &'a str,
}

#[derive(Responder, Debug, Serialize, Deserialize)]
pub struct UpdateShortDescriptionResponse {
    status: &'static str,
    //message: String,
}

#[post("/admin/api/settings/update-short-description", data = "<form>")]
pub fn update_short_description(
    cookies: &CookieJar<'_>,
    form: Form<UpdateShortDescription<'_>>,
) -> Json<UpdateShortDescriptionResponse> {
    if !is_authenticated(cookies) {
        return Json(UpdateShortDescriptionResponse {
            status: "error",
            //message: "Not authenticated",
        });
    }

    update_or_create_setting("short_description", form.description);

    Json(UpdateShortDescriptionResponse {
        status: "success",
        //message: "Post updated",
    })
}
