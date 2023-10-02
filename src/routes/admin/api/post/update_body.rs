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
pub struct UpdateBody<'a> {
    body: &'a str,
}

#[derive(Responder, Debug, Serialize, Deserialize)]
pub struct UpdateBodyResponse {
    status: &'static str,
    //message: String,
}

#[post("/admin/api/post/<id>/update-body", data = "<form>")]
pub fn update_body(
    id: i32,
    cookies: &CookieJar<'_>,
    form: Form<UpdateBody<'_>>,
) -> Json<UpdateBodyResponse> {
    if !is_authenticated(cookies) {
        return Json(UpdateBodyResponse {
            status: "error",
            //message: "Not authenticated",
        });
    }

    if get_post(id).is_none() {
        return Json(UpdateBodyResponse {
            status: "error",
            //message: "Post not found",
        });
    }

    use schema::posts::dsl::{body, id as post_id, posts};

    let updated_post = diesel::update(posts)
        .filter(post_id.eq(id))
        .set(body.eq(form.body))
        .execute(&mut db::connection());

    match updated_post {
        Ok(_) => {
            return Json(UpdateBodyResponse {
                status: "success",
                //message: "Post updated",
            });
        }
        Err(_) => {
            return Json(UpdateBodyResponse {
                status: "error",
                //message: "Error updating post",
            });
        }
    }
}
