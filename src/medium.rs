use feed_rs::{model::Feed, parser};

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
    let title = match feed.title {
        Some(title) => title.content,
        None => "N/A".to_owned(),
    };

    let entries: String = feed
        .entries
        .iter()
        .map(|entry| {
            let entry_title: String = match &entry.title {
                Some(title) => title.content,
                None => "N/A".to_string(),
            };

            return format!("{{\"title\": \"{}\"}},", entry_title);
        })
        .collect();

    return format!("{{\"title\": \"{}\", \"entries\": {}}}", title, entries);
}

pub async fn feed() -> String {
    match fetch_feed().await {
        Ok(xml) => collect_feed(parser::parse(xml.as_bytes()).unwrap()),
        Err(err) => handler(err),
    }
}
