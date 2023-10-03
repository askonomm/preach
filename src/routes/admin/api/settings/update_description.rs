use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{form::Form, http::CookieJar};
use serde::Deserialize;

use crate::utils::data::is_authenticated;
use crate::utils::data::update_or_create_setting;

#[derive(FromForm)]
pub struct UpdateDescription<'a> {
    description: &'a str,
}

#[derive(Responder, Debug, Serialize, Deserialize)]
pub struct UpdateDescriptionResponse {
    status: &'static str,
    //message: String,
}

#[post("/admin/api/settings/update-description", data = "<form>")]
pub fn update_description(
    cookies: &CookieJar<'_>,
    form: Form<UpdateDescription<'_>>,
) -> Json<UpdateDescriptionResponse> {
    if !is_authenticated(cookies) {
        return Json(UpdateDescriptionResponse {
            status: "error",
            //message: "Not authenticated",
        });
    }

    update_or_create_setting("description", form.description);

    Json(UpdateDescriptionResponse {
        status: "success",
        //message: "Post updated",
    })
}
