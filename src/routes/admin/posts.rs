use rocket_dyn_templates::{context, Template};

use crate::utils::get_posts;

#[get("/admin/posts")]
pub fn posts() -> Template {
    Template::render(
        "admin/posts",
        context! {
            posts: get_posts(),
        },
    )
}
