use diesel::prelude::*;
use rocket::http::CookieJar;

use crate::db;
use crate::schema;

pub fn is_authenticated(cookies: &CookieJar) -> bool {
    if cookies.get("auth_token").is_none() {
        return false;
    }

    use self::schema::users::dsl::{auth_token, id, users};

    let user_ids = users
        .filter(auth_token.eq(cookies.get("auth_token").unwrap().value()))
        .select(id)
        .load::<i32>(&mut db::connection());

    match user_ids {
        Ok(results) => !results.is_empty(),
        Err(_) => false,
    }
}
