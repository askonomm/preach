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
pub struct UpdateSlug<'a> {
    slug: &'a str,
}

#[derive(Responder, Debug, Serialize, Deserialize)]
pub struct UpdateSlugResponse {
    status: &'static str,
    //message: String,
}

#[post("/admin/api/post/<id>/update-slug", data = "<form>")]
pub fn update_slug(
    id: i32,
    cookies: &CookieJar<'_>,
    form: Form<UpdateSlug<'_>>,
) -> Json<UpdateSlugResponse> {
    if !is_authenticated(cookies) {
        return Json(UpdateSlugResponse {
            status: "error",
            //message: "Not authenticated",
        });
    }

    if get_post(id).is_none() {
        return Json(UpdateSlugResponse {
            status: "error",
            //message: "Post not found",
        });
    }

    use schema::posts::dsl::{id as post_id, posts, slug};

    let updated_post = diesel::update(posts)
        .filter(post_id.eq(id))
        .set(slug.eq(form.slug))
        .execute(&mut db::connection());

    match updated_post {
        Ok(_) => {
            return Json(UpdateSlugResponse {
                status: "success",
                //message: "Post updated",
            });
        }
        Err(_) => {
            return Json(UpdateSlugResponse {
                status: "error",
                //message: "Error updating post",
            });
        }
    }
}
