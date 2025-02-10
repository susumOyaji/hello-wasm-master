use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use web_sys::{HtmlElement, Request, RequestInit, RequestMode, Response, Element};
use wasm_bindgen_futures::JsFuture;
use web_sys::console;

#[derive(Debug, Serialize)]
struct ExtractedData {
    pub title: String,
    pub items: Vec<String>,
}

#[wasm_bindgen]
pub async fn fetch_and_parse_html(url: &str) -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().ok_or("No global `window` exists")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: Response = resp_value.dyn_into()?;

    let text_value = JsFuture::from(response.text()?).await?;
    let html_text: String = text_value.as_string().ok_or("Failed to get response text")?;

    console::log_1(&JsValue::from_str("Fetched HTML:"));
    console::log_1(&JsValue::from_str(&html_text));

    let document = window.document().ok_or("Should have a document on window")?;
    let parser = document.create_element("div")?.dyn_into::<HtmlElement>()?;
    parser.set_inner_html(&html_text);

    let mut extracted = ExtractedData {
        title: String::new(),
        items: vec![],
    };

    if let Some(title_elem) = parser.query_selector("b")? {
        extracted.title = title_elem.text_content().unwrap_or_default();
    }

    let uls = parser.query_selector_all("ul")?;
    for i in 0..uls.length() {
        if let Some(ul) = uls.item(i) {
            if let Some(ul_element) = ul.dyn_ref::<Element>() {
                let list_items = ul_element.query_selector_all("li")?;
                for j in 0..list_items.length() {
                    if let Some(li) = list_items.item(j) {
                        if let Some(li_element) = li.dyn_ref::<Element>() {
                            extracted.items.push(li_element.inner_html());
                        } else {
                            extracted.items.push(li.text_content().unwrap_or_default());
                        }
                    }
                }
            }
        }
    }

    console::log_1(&JsValue::from_str(&format!("Scraped Data: {:?}", extracted)));

    Ok(to_value(&extracted).map_err(|e| JsValue::from_str(&e.to_string()))?)
}
