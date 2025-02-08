//npx http-server
//npx http-server


use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

#[wasm_bindgen]
pub fn fizzbuzz(n: u32) -> String {
    match (n%3, n%5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        (_, _) => n.to_string(),
    }
}

async fn fetch_data(url: &str) -> Result<String, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}

#[wasm_bindgen(start)]
pub fn run() {
    log("Hello, world!");

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let app = document.get_element_by_id("app").unwrap();
    app.set_inner_html("Hello from Rust!");

    let p = document.create_element("p").unwrap();
    p.set_inner_html("色は匂へど散りぬるをsample");
    app.append_child(&p).unwrap();



    // スクレイピングの実行
    let url = "https://yoshizo.hatenablog.com/entry/microsoft-rewards-search-keyword-list/#movie";
    spawn_local(async move {
        match fetch_data(url).await {
            Ok(data) => {
                let p = document.create_element("b").unwrap();
                p.set_inner_html(&data);
                app.append_child(&p).unwrap();

                // pの内容をコンソールに出力
                log(&p.inner_html());
            }
            Err(err) => log(&format!("Error wasm: {:?}", err)),
        }
    });
}
