use rocket_dyn_templates::{context, Template};

use crate::utils::data::get_setting;

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "site/index",
        context! {
            site_title: get_setting("title")
        },
    )
}
