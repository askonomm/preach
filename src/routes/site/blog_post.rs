use rocket_dyn_templates::{context, Template};

use crate::{models::Post, utils::data::get_post_by_slug};

#[get("/blog/<slug>")]
pub fn blog_post(slug: &str) -> Template {
    let post = get_post_by_slug(slug);

    match post {
        Some(post) => Template::render(
            "site/blog_post",
            context! {
                post: post
            },
        ),
        None => Template::render(
            "site/404",
            context! {
                post: None::<Post>
            },
        ),
    }
}
