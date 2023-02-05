use feed_rs::{
    model::{Entry, Feed},
    parser,
};
use serde::Serialize;
extern crate regex;
use regex::Regex;

#[derive(Serialize, Debug)]
struct MyFeed {
    title: String,
    entries: Vec<MyEntry>,
}

#[derive(Serialize, Debug)]
struct MyEntry {
    slug: String,
    title: String,
    summary: String,
    content: String,
}

impl From<Entry> for MyEntry {
    fn from(value: Entry) -> Self {
        let title_value = value
            .title
            .map_or(String::from("Frank Mayer Blog"), |title_text| {
                title_text.content
            });

        let content_value = value.content.map_or(String::from("N/A"), |content| {
            content
                .body
                .map_or(String::from(""), |content| remove_medium_referrer(content))
        });

        let summary_value = value
            .summary
            .map_or("".to_owned(), |summary| summary.content);

        MyEntry {
            slug: make_slug(title_value.clone()),
            title: title_value,
            content: content_value,
            summary: summary_value,
        }
    }
}

impl From<Feed> for MyFeed {
    fn from(value: Feed) -> Self {
        MyFeed {
            title: value
                .title
                .map_or(String::from("N/A"), |title_text| title_text.content),
            entries: value.entries.into_iter().map(|a| a.into()).collect(),
        }
    }
}

fn make_slug(title: String) -> String {
    title.replace(" ", "-").to_lowercase()
}

fn remove_medium_referrer(html: String) -> String {
    let medium_referrer_pattern: Regex =
        Regex::new(r#"<img[^>]+https://medium\.com[^>]+>\s*$"#).unwrap();

    medium_referrer_pattern
        .replace(&html, String::from(""))
        .to_string()
}

async fn fetch_feed() -> Result<String, reqwest::Error> {
    let data: String = reqwest::Client::new()
        .get("https://medium.com/feed/@tsukinoko")
        .send()
        .await?
        .text()
        .await?;

    return Ok(data);
}

fn handler(_e: reqwest::Error) -> String {
    return String::from("error while fetching data");
}

fn collect_feed(feed: Feed) -> String {
    let feed: MyFeed = feed.into();
    // Todo Handle error
    serde_json::to_string(&feed).unwrap()
}

pub async fn feed() -> String {
    match fetch_feed().await {
        Ok(xml) => collect_feed(parser::parse(xml.as_bytes()).unwrap()),
        Err(err) => handler(err),
    }
}
