use wasm_bindgen::prelude::*;
use web_sys::{window, Request, RequestInit, Response};
use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct Article {
    title: String,
    urls: Vec<String>,
}

// Fetch keywords from the webpage (adjusted for WASM)
#[wasm_bindgen]
pub async fn fetch_movie_keywords() -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let url = "https://yoshizo.hatenablog.com/entry/microsoft-rewards-search-keyword-list/#movie";

    // Fetch the HTML content
    let window = window().ok_or_else(|| JsValue::from_str("window not available"))?;
    let document = window.document().ok_or_else(|| JsValue::from_str("document not available"))?;

    let response = window.fetch_with_str(url).await.map_err(|err| JsValue::from_str(&format!("Failed to fetch page: {}", err)))?;
    let text = response.text().await.map_err(|err| JsValue::from_str(&format!("Failed to read response: {}", err)))?;

    // Parsing HTML and extracting content
    let document = Html::parse_document(&text);
    let title_b_selector = Selector::parse("b").unwrap();
    let ul_selector = Selector::parse("ul").unwrap();
    let li_selector = Selector::parse("li").unwrap();
    let b_selector = Selector::parse("ul li b").unwrap();

    let mut articles = Vec::new();

    for title_b in document.select(&title_b_selector) {
        let title_text = title_b.text().collect::<Vec<_>>().join("").trim().to_string();
        let mut urls = Vec::new();

        let mut sibling = title_b.next_sibling();
        while let Some(node) = sibling {
            if let Some(element) = ElementRef::wrap(node) {
                if element.value().name() == "ul" {
                    for li in element.select(&li_selector) {
                        for b_tag in li.select(&b_selector) {
                            let text = b_tag.text().collect::<Vec<_>>().join("").trim().to_string();
                            urls.push(text);
                        }
                    }
                    break;
                }
            }
            sibling = node.next_sibling();
        }

        if !urls.is_empty() {
            articles.push(Article { title: title_text, urls });
        }
    }

    JsValue::from_serde(&articles).map_err(|err| JsValue::from_str(&format!("Serialization error: {}", err)))
}
