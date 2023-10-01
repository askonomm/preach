#[get("/blog")]
pub fn blog() -> &'static str {
    "Hello, blog page!"
}
