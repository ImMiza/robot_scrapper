use std::collections::HashSet;
use std::env;
use reqwest::Url;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Missings arguments");
    }

    let mut url_list: HashSet<String> = HashSet::new();

    let default_url = args.get(1).unwrap().clone();
    start_scrapping(default_url, &mut url_list).await.expect("Error while scrapping");

    // http client

}

async fn start_scrapping(default_url: String, url_list: &mut HashSet<String>) -> Result<(), &'static str> {
    let result = fetch_url(&default_url).await;
    println!("List size : {}", &result.len());
    url_list.extend(result);

    while !url_list.is_empty() {
        if let Some(url) = url_list.iter().next().cloned() {
            let result = fetch_url(&url).await;
            println!("Found : {}", result.len());
            url_list.extend(result);
            url_list.remove(&url);
        }
        println!("List size : {}", url_list.len());
    }

    Ok(())
}

async fn fetch_url(url: &String) -> HashSet<String> {
    if let Ok(res) = reqwest::get(url).await {
        if let Ok(body) = res.text().await {
            if let Ok(urls) = extract_urls(Html::parse_document(&*body)).await {
                return urls;
            }
        }
    }
    HashSet::new()
}

async fn extract_urls(document: Html) -> Result<HashSet<String>, &'static str> {
    let mut urls: HashSet<String> = HashSet::new();
    let selector = Selector::parse("a[href]").unwrap();
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(url) = Url::parse(href) {
                println!("{}", href);
                urls.insert(href.to_string());
            }
        }
    }
    Ok(urls)
}
