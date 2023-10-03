use rocket_dyn_templates::{context, Template};

use crate::{
    models::Post,
    utils::data::{get_post_by_slug, get_setting},
};

#[get("/blog/<slug>")]
pub fn blog_post(slug: &str) -> Template {
    let post = get_post_by_slug(slug);

    match post {
        Some(post) => Template::render(
            "site/blog_post",
            context! {
                active_page: "blog",
                site_image: get_setting("image"),
                site_title: get_setting("title"),
                post: post
            },
        ),
        None => Template::render(
            "site/404",
            context! {
                site_image: get_setting("image"),
                site_title: get_setting("title"),
                post: None::<Post>
            },
        ),
    }
}
