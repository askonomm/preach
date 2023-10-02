use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::{form::Form, http::CookieJar};
use serde::Deserialize;

use crate::utils::date::date_str_to_system_time;
use crate::{
    db, schema,
    utils::data::{get_post, is_authenticated},
};

#[derive(FromForm)]
pub struct UpdateDate<'a> {
    published_at: &'a str,
}

#[derive(Responder, Debug, Serialize, Deserialize)]
pub struct UpdateDateResponse {
    status: &'static str,
    //message: String,
}

#[post("/admin/api/post/<id>/update-date", data = "<form>")]
pub fn update_date(
    id: i32,
    cookies: &CookieJar<'_>,
    form: Form<UpdateDate<'_>>,
) -> Json<UpdateDateResponse> {
    if !is_authenticated(cookies) {
        return Json(UpdateDateResponse {
            status: "error",
            //message: "Not authenticated",
        });
    }

    if get_post(id).is_none() {
        return Json(UpdateDateResponse {
            status: "error",
            //message: "Post not found",
        });
    }

    use schema::posts::dsl::{id as post_id, posts, published_at};

    let updated_post = diesel::update(posts)
        .filter(post_id.eq(id))
        .set(published_at.eq(date_str_to_system_time(form.published_at)))
        .execute(&mut db::connection());

    match updated_post {
        Ok(_) => {
            return Json(UpdateDateResponse {
                status: "success",
                //message: "Post updated",
            });
        }
        Err(_) => {
            return Json(UpdateDateResponse {
                status: "error",
                //message: "Error updating post",
            });
        }
    }
}
