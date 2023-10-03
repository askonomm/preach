use std::time::SystemTime;

use crate::db;
use crate::models;
use crate::schema;
use crate::utils::conversion::post_to_displayable_post;
use bcrypt::verify;
use diesel::prelude::*;
use rocket::http::CookieJar;

use super::conversion::DisplayablePost;

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

pub fn clear_auth_token(auth_token: &str) {
    use self::schema::users::dsl::{auth_token as user_auth_token, users};

    diesel::update(users.filter(user_auth_token.eq(auth_token)))
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

pub fn get_posts() -> Vec<DisplayablePost> {
    use self::schema::posts::dsl::{id, posts};

    let all_posts = posts
        .order(id.desc())
        .select(models::Post::as_select())
        .load::<models::Post>(&mut db::connection());

    match all_posts {
        Ok(all_posts) => all_posts
            .into_iter()
            .map(|post| {
                return post_to_displayable_post(post);
            })
            .collect(),
        Err(_) => vec![],
    }
}

pub fn get_published_posts() -> Vec<DisplayablePost> {
    use self::schema::posts::dsl::{id, posts, published_status};

    let all_posts = posts
        .filter(published_status.eq("published"))
        .order(id.desc())
        .select(models::Post::as_select())
        .load::<models::Post>(&mut db::connection());

    match all_posts {
        Ok(all_posts) => {
            return all_posts
                .into_iter()
                .map(|post| {
                    return post_to_displayable_post(post);
                })
                .collect();
        }
        Err(_) => vec![],
    }
}

pub fn get_post(id: i32) -> Option<DisplayablePost> {
    use self::schema::posts::dsl::{id as post_id, posts};

    let all_posts = posts
        .filter(post_id.eq(id))
        .select(models::Post::as_select())
        .load::<models::Post>(&mut db::connection());

    match all_posts {
        Ok(all_posts) => {
            if all_posts.is_empty() {
                return None;
            } else {
                return Some(post_to_displayable_post(all_posts[0].clone()));
            }
        }
        Err(_) => None,
    }
}

pub fn get_post_by_slug(slug: &str) -> Option<DisplayablePost> {
    use self::schema::posts::dsl::{posts, slug as post_slug};

    let all_posts = posts
        .filter(post_slug.eq(slug))
        .select(models::Post::as_select())
        .load::<models::Post>(&mut db::connection());

    match all_posts {
        Ok(all_posts) => {
            if all_posts.is_empty() {
                return None;
            } else {
                return Some(post_to_displayable_post(all_posts[0].clone()));
            }
        }
        Err(_) => None,
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

pub fn update_or_create_setting(key: &str, value: &str) {
    use schema::site_info::dsl::{created_at, entry, name, site_info, updated_at};

    let setting = site_info
        .filter(name.eq(key))
        .select(name)
        .load::<String>(&mut db::connection());

    match setting {
        Ok(setting) => {
            if setting.is_empty() {
                diesel::insert_into(schema::site_info::table)
                    .values((
                        name.eq(key),
                        entry.eq(value),
                        updated_at.eq(SystemTime::now()),
                        created_at.eq(SystemTime::now()),
                    ))
                    .execute(&mut db::connection())
                    .unwrap();
            } else {
                diesel::update(site_info.filter(name.eq(key)))
                    .set((entry.eq(value), updated_at.eq(SystemTime::now())))
                    .execute(&mut db::connection())
                    .unwrap();
            }
        }
        Err(_) => {}
    }
}

pub fn get_setting(key: &str) -> Option<String> {
    use schema::site_info::dsl::{entry, name, site_info};

    let setting = site_info
        .filter(name.eq(key))
        .select(entry)
        .load::<String>(&mut db::connection());

    match setting {
        Ok(setting) => {
            if setting.is_empty() {
                return None;
            } else {
                return Some(setting[0].clone());
            }
        }
        Err(_) => None,
    }
}
