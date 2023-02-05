use axum::http::{Response, StatusCode};
use serde::{Deserialize, Serialize};
use simple_xml_builder::XMLElement;
extern crate regex;

#[derive(Serialize, Deserialize, Debug)]
struct WakatimeLanguages {
    data: WakatimeLanguagesData,
}

#[derive(Serialize, Deserialize, Debug)]
struct WakatimeLanguage {
    name: String,
    total_seconds: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct WakatimeLanguagesData {
    is_up_to_date: bool,
    languages: Vec<WakatimeLanguage>,
    status: String,
}

async fn fetch_wakatime_languages() -> Result<WakatimeLanguagesData, reqwest::Error> {
    let resp_obj: WakatimeLanguages = reqwest::Client::new()
        .get("https://wakatime.com/api/v1/users/tsukinoko/insights/languages/last_7_days")
        .send()
        .await?
        .json()
        .await?;

    return Ok(resp_obj.data);
}

fn make_background() -> XMLElement {
    let mut rect_el = XMLElement::new("rect");
    rect_el.add_attribute("x", "0");
    rect_el.add_attribute("y", "0");
    rect_el.add_attribute("width", "100%");
    rect_el.add_attribute("height", "100%");
    rect_el.add_attribute("fill", "#0d1117");
    return rect_el;
}

fn make_title(text: &str) -> XMLElement {
    let mut text_el = XMLElement::new("text");
    text_el.add_attribute("class", "title");
    text_el.add_attribute("x", "0");
    text_el.add_attribute("y", "0");
    text_el.add_attribute("fill", "#aa51f8");
    text_el.add_text(text);

    let mut g_el = XMLElement::new("g");
    g_el.add_attribute("transform", "translate(25, 35)");
    g_el.add_child(text_el);

    return g_el;
}

fn make_style() -> XMLElement {
    let mut style_el = XMLElement::new("style");
    style_el.add_text(".title {font: 600 18px 'Segoe UI', Ubuntu, Sans-Serif;}");
    return style_el;
}

pub async fn wakatime() -> Response<String> {
    let mut svg_el = XMLElement::new("svg");
    svg_el.add_attribute("xmlns", "http://www.w3.org/2000/svg");
    svg_el.add_attribute("role", "img");
    svg_el.add_attribute("viewBox", "0 0 495 240");
    svg_el.add_attribute("height", "240");
    svg_el.add_attribute("width", "495");
    svg_el.add_attribute("fill", "none");

    svg_el.add_child(make_style());
    svg_el.add_child(make_background());
    svg_el.add_child(make_title("Most Used Languages Last Week"));

    let mut lang_list_el = XMLElement::new("ul");
    match fetch_wakatime_languages().await {
        Ok(data) => {
            for lang in data.languages {
                let mut lang_el = XMLElement::new("li");
                lang_el.add_text(lang.name);
                lang_list_el.add_child(lang_el);
            }
        }
        Err(err) => {
            let mut err_el = XMLElement::new("li");
            err_el.add_text(err.to_string());
            lang_list_el.add_child(err_el);
        }
    }
    svg_el.add_child(lang_list_el);

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/svg+xml; charset=utf-8")
        .body(svg_el.to_string())
        .unwrap()
}
