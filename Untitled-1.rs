//use flutter_rust_bridge::frb;
use reqwest;
use scraper::{Html, Selector, ElementRef};
use serde::Serialize;
use std::error::Error;

#[derive(Serialize, Debug)]
pub struct Article {
    title: String,
    urls: Vec<String>,
}

//#[frb]  // Flutter から呼び出せる関数
pub async fn fetch_movie_keywords() -> Result<Vec<Article>, String> {
    let url = "https://yoshizo.hatenablog.com/entry/microsoft-rewards-search-keyword-list/#movie";  // スクレイピング対象のURLを指定
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to fetch page: {}", e))?
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    let document = Html::parse_document(&response);

    // **タイトルの <b> タグを取得**
    let title_b_selector = Selector::parse("b").unwrap();
    let ul_selector = Selector::parse("ul").unwrap();
    let li_selector = Selector::parse("li").unwrap();
    let b_selector = Selector::parse("ul li b").unwrap();

    let mut articles = Vec::new();

    // **タイトルごとに記事を保存**
    for title_b in document.select(&title_b_selector) {
        let title_text = title_b.text().collect::<Vec<_>>().join("").trim().to_string();
        let mut urls = Vec::new();

        // **対応する <ul> を取得**
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

        // **記事をリストに追加**
        if !urls.is_empty() {
            articles.push(Article {
                title: title_text,
                urls,
            });
        }
    }

    // **デバッグ用にコンソールに出力**
    println!("{:?}", articles);
    //println!("{}", serde_json::to_string_pretty(&articles).unwrap());

    Ok(articles)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match fetch_movie_keywords().await {
        Ok(articles) => {
            for article in articles {
                println!("Title: {}", article.title);
                for url in article.urls {
                    println!("URL: {}", url);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    Ok(())
}
