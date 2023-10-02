use rocket_dyn_templates::{context, Template};

use crate::utils::data::{get_published_posts, get_setting};

#[get("/blog")]
pub fn blog() -> Template {
    Template::render(
        "site/blog",
        context! {
            site_title: get_setting("title"),
            posts: get_published_posts()
        },
    )
}
