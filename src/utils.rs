use bcrypt::verify;
use diesel::prelude::*;
use rocket::http::CookieJar;

use crate::db;
use crate::models;
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

pub fn authenticates(email: &str, password: &str) -> bool {
    use self::schema::users::dsl::{email as user_email, password as user_password, users};

    let all_users = users
        .filter(user_email.eq(email))
        .select(user_password)
        .load::<String>(&mut db::connection());

    match all_users {
        Ok(all_users) => !all_users.is_empty() && verify(password, &all_users[0]).unwrap(),
        Err(_) => false,
    }
}

pub fn set_auth_token(email: &str, auth_token: &str) {
    use self::schema::users::dsl::{auth_token as user_auth_token, email as user_email, users};

    diesel::update(users.filter(user_email.eq(email)))
        .set(user_auth_token.eq(auth_token))
        .execute(&mut db::connection())
        .unwrap();
}

pub fn clear_auth_token(email: &str) {
    use self::schema::users::dsl::{auth_token as user_auth_token, email as user_email, users};

    diesel::update(users.filter(user_email.eq(email)))
        .set(user_auth_token.eq(""))
        .execute(&mut db::connection())
        .unwrap();
}

pub fn is_setup() -> bool {
    use self::schema::users::dsl::{id, users};

    let user_ids = users.select(id).load::<i32>(&mut db::connection());

    match user_ids {
        Ok(results) => !results.is_empty(),
        Err(_) => false,
    }
}

pub fn get_posts() -> Vec<models::Post> {
    use self::schema::posts::dsl::{id, posts};

    let all_posts = posts
        .order(id.desc())
        .select(models::Post::as_select())
        .load::<models::Post>(&mut db::connection());

    match all_posts {
        Ok(all_posts) => all_posts,
        Err(_) => vec![],
    }
}

pub fn get_user_id_by_auth_token(auth_token: String) -> Option<i32> {
    use self::schema::users::dsl::{auth_token as user_auth_token, id, users};

    let user_ids = users
        .filter(user_auth_token.eq(auth_token))
        .select(id)
        .load::<i32>(&mut db::connection());

    match user_ids {
        Ok(results) => {
            if results.is_empty() {
                return None;
            } else {
                return Some(results[0]);
            }
        }
        Err(_) => None,
    }
}
