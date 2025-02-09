use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use web_sys::{HtmlElement};
use web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen_futures::JsFuture;

#[derive(Serialize)]
struct ExtractedData {
    title: String,
    items: Vec<String>,
}

#[wasm_bindgen]
pub fn fetch_and_parse_html(url: &str) -> Result<JsValue, JsValue> {

     // リクエストを作成
     let opts = RequestInit::new();
     opts.method("GET");
     opts.mode(RequestMode::Cors); // CORS に注意
 
     let request = Request::new_with_str_and_init(&url, &opts)?;
 
     // fetch API を呼び出す
     let window = web_sys::window().unwrap();
     let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
 
 
    let window = web_sys::window().ok_or("No global `window` exists")?;
    let document = window.document().ok_or("Should have a document on window")?;
    
    let parser = document
        .create_element("div")?
        .dyn_into::<HtmlElement>()?;
    parser.set_inner_html(html);
    
    let mut extracted = ExtractedData {
        title: String::new(),
        items: vec![],
    };

    if let Some(title_elem) = parser.query_selector("b")? {
        extracted.title = title_elem.text_content().unwrap_or_default();
    }

    if let Some(ul_elem) = parser.query_selector("ul")? {
        let list_items = ul_elem.query_selector_all("li")?;
        for i in 0..list_items.length() {
            if let Some(li) = list_items.item(i) {
                extracted.items.push(li.text_content().unwrap_or_default());
            }
        }
    }

    // `serde-wasm-bindgen` を使用して `JsValue` に変換
    Ok(to_value(&extracted).map_err(|e| JsValue::from_str(&e.to_string()))?)
}
