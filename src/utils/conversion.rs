use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::Post;

use super::date::system_time_to_date_str;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DisplayablePost {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
    pub raw_body: String,
    pub published_status: String,
    pub published_at: String,
    pub published_at_pretty: String,
    pub published_at_pretty_short: String,
}

pub fn post_to_displayable_post(post: Post) -> DisplayablePost {
    let published_at = system_time_to_date_str(post.published_at)
        .split_at(10)
        .0
        .to_string();
    let published_at_datetime: DateTime<Utc> = post.published_at.into();

    DisplayablePost {
        id: post.id,
        title: post.title,
        slug: post.slug,
        body: markdown::to_html(&post.body),
        raw_body: post.body,
        published_status: post.published_status,
        published_at,
        published_at_pretty: published_at_datetime.format("%B %e, %Y").to_string(),
        published_at_pretty_short: published_at_datetime.format("%b %d, %Y").to_string(),
    }
}
