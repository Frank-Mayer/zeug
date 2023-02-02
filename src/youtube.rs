use axum::response::Redirect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Live {
    id: String,
    #[serde(rename = "isRecording")]
    is_recording: bool,
}

async fn live_internal() -> Result<Live, reqwest::Error> {
    let data: Live = reqwest::Client::new()
        .get("https://zeug-5c731-default-rtdb.europe-west1.firebasedatabase.app/youtubeLive.json")
        .send()
        .await?
        .json()
        .await?;

    Ok(data)
}

pub async fn live() -> Redirect {
    let res = live_internal().await;

    match res {
        Ok(data) => Redirect::temporary(&*format!("https://www.youtube-nocookie.com/embed/{}?autoplay=1", data.id)),
        Err(_) => Redirect::temporary("about:blank"),
    }
}
