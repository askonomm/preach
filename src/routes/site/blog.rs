use rocket_dyn_templates::{context, Template};

use crate::utils::data::get_published_posts;

#[get("/blog")]
pub fn blog() -> Template {
    Template::render(
        "site/blog",
        context! {
            posts: get_published_posts()
        },
    )
}
