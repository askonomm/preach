use rocket_dyn_templates::{context, Template};

use crate::utils::data::get_setting;

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "site/index",
        context! {
            active_page: "about",
            site_image: get_setting("image"),
            site_title: get_setting("title"),
            site_description: markdown::to_html(&get_setting("description").unwrap_or("".to_string())),
        },
    )
}
