use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{form::Form, http::CookieJar};
use serde::Deserialize;

use crate::{
    db, schema,
    utils::data::{get_post, is_authenticated},
};

#[derive(FromForm)]
pub struct UpdateStatus<'a> {
    published_status: &'a str,
}

#[derive(Responder, Debug, Serialize, Deserialize)]
pub struct UpdateStatusResponse {
    status: &'static str,
    //message: String,
}

#[post("/admin/api/post/<id>/update-status", data = "<form>")]
pub fn update_status(
    id: i32,
    cookies: &CookieJar<'_>,
    form: Form<UpdateStatus<'_>>,
) -> Json<UpdateStatusResponse> {
    if !is_authenticated(cookies) {
        return Json(UpdateStatusResponse {
            status: "error",
            //message: "Not authenticated",
        });
    }

    if get_post(id).is_none() {
        return Json(UpdateStatusResponse {
            status: "error",
            //message: "Post not found",
        });
    }

    use schema::posts::dsl::{id as post_id, posts, published_status};

    let updated_post = diesel::update(posts)
        .filter(post_id.eq(id))
        .set(published_status.eq(form.published_status))
        .execute(&mut db::connection());

    match updated_post {
        Ok(_) => {
            return Json(UpdateStatusResponse {
                status: "success",
                //message: "Post updated",
            });
        }
        Err(_) => {
            return Json(UpdateStatusResponse {
                status: "error",
                //message: "Error updating post",
            });
        }
    }
}
