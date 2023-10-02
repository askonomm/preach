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
pub struct UpdateTitle<'a> {
    title: &'a str,
}

#[derive(Responder, Debug, Serialize, Deserialize)]
pub struct UpdateTitleResponse {
    status: &'static str,
    //message: String,
}

#[post("/admin/api/post/<id>/update-title", data = "<form>")]
pub fn update_title(
    id: i32,
    cookies: &CookieJar<'_>,
    form: Form<UpdateTitle<'_>>,
) -> Json<UpdateTitleResponse> {
    if !is_authenticated(cookies) {
        return Json(UpdateTitleResponse {
            status: "error",
            //message: "Not authenticated",
        });
    }

    if get_post(id).is_none() {
        return Json(UpdateTitleResponse {
            status: "error",
            //message: "Post not found",
        });
    }

    use schema::posts::dsl::{id as post_id, posts, title};

    let updated_post = diesel::update(posts)
        .filter(post_id.eq(id))
        .set(title.eq(form.title))
        .execute(&mut db::connection());

    match updated_post {
        Ok(_) => {
            return Json(UpdateTitleResponse {
                status: "success",
                //message: "Post updated",
            });
        }
        Err(_) => {
            return Json(UpdateTitleResponse {
                status: "error",
                //message: "Error updating post",
            });
        }
    }
}
