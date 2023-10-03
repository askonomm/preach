use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{form::Form, http::CookieJar};
use serde::Deserialize;

use crate::utils::data::is_authenticated;
use crate::utils::data::update_or_create_setting;

#[derive(FromForm)]
pub struct UpdateImage<'a> {
    image: &'a str,
}

#[derive(Responder, Debug, Serialize, Deserialize)]
pub struct UpdateImageResponse {
    status: &'static str,
    //message: String,
}

#[post("/admin/api/settings/update-image", data = "<form>")]
pub fn update_image(
    cookies: &CookieJar<'_>,
    form: Form<UpdateImage<'_>>,
) -> Json<UpdateImageResponse> {
    if !is_authenticated(cookies) {
        return Json(UpdateImageResponse {
            status: "error",
            //message: "Not authenticated",
        });
    }

    update_or_create_setting("image", form.image);

    Json(UpdateImageResponse {
        status: "success",
        //message: "Post updated",
    })
}
