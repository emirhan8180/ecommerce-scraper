use anyhow::Result;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<()> {
    let response = reqwest::Client::new()
        .get("https://www.scrapingcourse.com/ecommerce")
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&response);
    let selector = Selector::parse("#product-list > li :is(img, h2, bdi)").unwrap();

    for element in document.select(&selector) {
        println!("{element:?}");
    }

    Ok(())
}
