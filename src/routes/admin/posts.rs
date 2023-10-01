#[get("/admin/posts")]
pub fn posts() -> &'static str {
    "Hello, admin posts!"
}
