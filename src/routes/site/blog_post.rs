#[get("/blog/<slug>")]
pub fn blog_post(slug: &str) -> &'static str {
    "Hello, post page!"
}
