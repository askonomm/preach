use diesel::prelude::*;
use rocket::{http::CookieJar, response::Redirect};
use rocket_dyn_templates::{context, Template};
use std::time::SystemTime;

use crate::{
    db, models, schema,
    utils::{get_posts, get_user_id_by_auth_token, is_authenticated},
};

#[derive(Responder)]
pub enum PostsResponse {
    Template(Template),
    Redirect(Redirect),
}

#[get("/admin/posts")]
pub fn posts(cookies: &CookieJar<'_>) -> PostsResponse {
    if !is_authenticated(cookies) {
        return PostsResponse::Redirect(Redirect::to("/admin/login"));
    }

    PostsResponse::Template(Template::render(
        "admin/posts",
        context! {
            posts: get_posts(),
        },
    ))
}

#[get("/admin/posts/new")]
pub fn new_post(cookies: &CookieJar<'_>) -> Redirect {
    if !is_authenticated(cookies) {
        return Redirect::to("/admin/login");
    }

    let auth_token = cookies.get("auth_token").unwrap().value().to_string();

    let new_post = models::NewPost {
        title: "Untitled",
        slug: "untitled",
        body: "",
        published_status: "draft",
        user_id: get_user_id_by_auth_token(auth_token).unwrap(),
        published_at: SystemTime::now(),
        created_at: SystemTime::now(),
        updated_at: SystemTime::now(),
    };

    let inserted_id = diesel::insert_into(schema::posts::table)
        .values(&new_post)
        .returning(schema::posts::id)
        .get_result::<i32>(&mut db::connection());

    Redirect::to(format!("/admin/posts/edit/{}", inserted_id.unwrap()))
}

#[get("/admin/posts/edit/<id>")]
pub fn edit_post(id: i32, cookies: &CookieJar<'_>) -> PostsResponse {
    if !is_authenticated(cookies) {
        return PostsResponse::Redirect(Redirect::to("/admin/login"));
    }

    PostsResponse::Template(Template::render("admin/edit_post", context! {}))
}
