use std::collections::HashSet;
use std::env;
use reqwest::header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, HeaderMap, USER_AGENT};
use reqwest::Url;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Missings arguments");
    }

    let mut url_list: HashSet<String> = HashSet::new();

    for arg in args.iter().skip(1) {
        println!("{}", arg);
        url_list.insert(arg.clone());
    }

    start_scrapping(&mut url_list).await.expect("Error while scrapping");

    // http client

}

async fn start_scrapping(url_list: &mut HashSet<String>) -> Result<(), &'static str> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:104.0) Gecko/20100101 Firefox/104.0".parse().unwrap());
    headers.insert(ACCEPT_LANGUAGE, "en-US,en;q=0.5".parse().unwrap());

    let mut i = 0;
    while !url_list.is_empty() {
        if let Some(url) = url_list.iter().next().cloned() {
            let result = fetch_url(&url, &headers).await;
            url_list.extend(result);
            url_list.remove(&url);
        }
        println!("iter : {}", i);
        println!("List size : {}", url_list.len());
        println!("List : {:?}\n", url_list.iter().take(5).collect::<Vec<&String>>());

        i = i + 1;
    }

    Ok(())
}

async fn fetch_url(url: &String, header: &HeaderMap) -> HashSet<String> {
    let client = reqwest::Client::new();
    if let Ok(res) = client.head(url).headers(header.clone()).send().await {
        if let Some(content_type) = res.headers().get(reqwest::header::CONTENT_TYPE) {
            if content_type.to_str().unwrap().contains("text/html") {
                // Continue seulement si le Content-Type est text/html
                if let Ok(res) = client.get(url).headers(header.clone()).send().await {
                    if let Ok(body) = res.text().await {
                        if let Ok(urls) = extract_urls(Html::parse_document(&*body)).await {
                            return urls;
                        }
                    }
                }
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
                urls.insert(href.to_string());
            }
        }
    }
    Ok(urls)
}
