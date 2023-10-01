use rocket_dyn_templates::{context, Template};

#[get("/admin/login")]
pub fn login() -> Template {
    let context = context! {};

    Template::render("admin/login", &context)
}
