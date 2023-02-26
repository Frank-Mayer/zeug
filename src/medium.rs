use feed_rs::{
    model::{Entry, Feed},
    parser,
};
use serde::Serialize;
extern crate regex;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Serialize, Debug)]
struct StoryFeed {
    title: String,
    entries: Vec<StoryEntry>,
}

#[derive(Serialize, Debug)]
struct StoryEntry {
    slug: String,
    title: String,
    summary: String,
    content: String,
    published: String,
    keywords: Vec<String>,
    preview_img: String,
}

lazy_static! {
    static ref IMG: Regex = Regex::new(r#"<img\s[^>]*src\s*="([^"]+)"[^>]*/?>"#).unwrap();
    static ref ARTICLE_ID: Regex = Regex::new(r#"[^/]{4,}$"#).unwrap();
    static ref WHITESPACE: Regex = Regex::new(r#"\s+"#).unwrap();
}

fn empty_string() -> String {
    return String::from("");
}

impl From<Entry> for StoryEntry {
    fn from(value: Entry) -> Self {
        let title_value = value
            .title
            .map_or(String::from("Frank Mayer Blog"), |title_text| {
                title_text.content
            });

        let content_value = value.content.map_or(String::from("N/A"), |content| {
            content.body.unwrap_or_else(empty_string)
        });

        let summary_value = value
            .summary
            .map_or_else(empty_string, |summary| summary.content);

        let slug_value = make_slug(title_value.as_str(), value.id.as_str());

        let publish_date = value
            .published
            .map_or_else(empty_string, |date| date.format("%Y-%m-%d").to_string());

        let keywords_value: Vec<String> = value
            .categories
            .iter()
            .map(|el| el.label.to_owned().unwrap_or_else(|| el.term.clone()))
            .collect();

        // find image element and use source for preview image
        let preview_img_value =
            IMG.captures(content_value.as_str())
                .map_or_else(empty_string, |capt| {
                    capt.get(1)
                        .map_or_else(empty_string, |match_obj| match_obj.as_str().to_string())
                });

        StoryEntry {
            slug: slug_value,
            title: title_value,
            content: content_value,
            summary: summary_value,
            published: publish_date,
            keywords: keywords_value,
            preview_img: preview_img_value,
        }
    }
}

impl From<Feed> for StoryFeed {
    fn from(value: Feed) -> Self {
        StoryFeed {
            title: value
                .title
                .map_or(String::from("N/A"), |title_text| title_text.content),
            entries: value.entries.into_iter().map(|a| a.into()).collect(),
        }
    }
}

fn make_slug(title: &str, permalink: &str) -> String {
    let slug_title = WHITESPACE.replace_all(title, "-").to_lowercase();

    ARTICLE_ID
        .find(permalink)
        .map(|m| m.as_str())
        .map(|id| format!("{}-{}", slug_title, id))
        .unwrap_or_else(|| slug_title)
}

async fn fetch_feed() -> Result<String, reqwest::Error> {
    let data: String = reqwest::get("https://medium.com/feed/@tsukinoko")
        .await?
        .text()
        .await?;

    return Ok(data);
}

fn default_feed() -> String {
    String::from(r#"{"title":"N/A","entries":[]}"#)
}

pub async fn feed() -> String {
    match fetch_feed().await {
        Ok(rss_str) => match parser::parse(rss_str.as_bytes()) {
            Ok(rss) => {
                let story_feed: StoryFeed = rss.into();
                serde_json::to_string(&story_feed).unwrap_or_else(|_| default_feed())
            }
            Err(_) => default_feed(),
        },
        Err(_) => default_feed(),
    }
}
