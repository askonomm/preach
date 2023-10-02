use std::time::SystemTime;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub user_id: i32,
    pub published_status: String,
    pub published_at: SystemTime,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub user_id: i32,
    pub published_status: &'a str,
    pub published_at: SystemTime,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub auth_token: String,
    pub remember_token: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub auth_token: &'a str,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
