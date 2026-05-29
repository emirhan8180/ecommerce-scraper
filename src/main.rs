use anyhow::Result;
use scraper::{Html, Selector};

#[derive(Debug)]
struct ProductData<'a> {
    url: &'a str,
    image: &'a str,
    name: String,
    price: f32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let response = reqwest::Client::new()
        .get("https://www.scrapingcourse.com/ecommerce")
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&response);

    let product_selector = Selector::parse("#product-list > li").unwrap();

    let link_selector = Selector::parse("a").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let title_selector = Selector::parse("h2").unwrap();
    let price_selector = Selector::parse(".price").unwrap();

    let data: Vec<_> = document
        .select(&product_selector)
        .filter_map(|product| {
            let a = product.select(&link_selector).next()?;
            let img = product.select(&img_selector).next()?;
            let h2 = product.select(&title_selector).next()?;
            let bdi = product.select(&price_selector).next()?;

            Some(ProductData {
                url: a.value().attr("href")?,
                image: img.value().attr("src")?,
                name: h2.text().collect(),
                price: bdi
                    .text()
                    .collect::<String>()
                    .replace('$', "")
                    .trim()
                    .parse()
                    .ok()?,
            })
        })
        .collect();

    println!("{data:?}");

    Ok(())
}
