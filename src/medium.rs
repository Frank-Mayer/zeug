use feed_rs::{
    model::{Entry, Feed},
    parser,
};
use serde::Serialize;

#[derive(Serialize, Debug)]
struct MyFeed {
    title: String,
    entries: Vec<MyEntry>,
}

#[derive(Serialize, Debug)]
struct MyEntry {
    title: String,
    summary: String,
    content: String,
}

impl From<Entry> for MyEntry {
    fn from(value: Entry) -> Self {
        let title_value = value
            .title
            .map_or("N/A".to_owned(), |title_text| title_text.content);

        let content_value = value.content.map_or("N/A".to_owned(), |content| {
            content.body.unwrap_or("N/A".to_owned())
        });

        let summary_value = value
            .summary
            .map_or("N/A".to_owned(), |summary| summary.content);

        MyEntry {
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
                .map_or("N/A".to_owned(), |title_text| title_text.content),
            entries: value.entries.into_iter().map(|a| a.into()).collect(),
        }
    }
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
    return "error while fetching data".to_owned();
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
