use std::fs;

use anyhow::Result;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

struct ProductDataSelectors<'a> {
    link: &'a Selector,
    img: &'a Selector,
    title: &'a Selector,
    price: &'a Selector,
}

#[derive(Serialize, Deserialize)]
struct ProductData {
    url: String,
    image: String,
    name: String,
    price: f32,
}

impl ProductData {
    fn get_from_element(product: ElementRef, selectors: &ProductDataSelectors) -> Option<Self> {
        let a = product.select(selectors.link).next()?;
        let img = product.select(selectors.img).next()?;
        let h2 = product.select(selectors.title).next()?;
        let price = product.select(selectors.price).next()?;

        let price: String = price.text().collect();

        Some(ProductData {
            url: a.value().attr("href")?.to_string(),
            image: img.value().attr("src")?.to_string(),
            name: h2.text().collect(),
            price: price.replace('$', "").trim().parse().ok()?,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let product_selector = Selector::parse("#product-list > li").unwrap();

    let selectors = ProductDataSelectors {
        link: &Selector::parse("a").unwrap(),
        img: &Selector::parse("img").unwrap(),
        title: &Selector::parse("h2").unwrap(),
        price: &Selector::parse(".price").unwrap(),
    };

    let client = reqwest::Client::new();

    let mut data = vec![];
    for page in 0..13 {
        let url = format!("https://www.scrapingcourse.com/ecommerce/page/{page}");
        let response = client.get(url).send().await?.text().await?;

        let document = Html::parse_document(&response);
        let products = document.select(&product_selector);

        data.extend(
            products.filter_map(|product| ProductData::get_from_element(product, &selectors)),
        );
    }

    fs::write("./data.json", serde_json::to_string(&data)?)?;

    Ok(())
}
