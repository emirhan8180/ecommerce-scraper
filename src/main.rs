use std::fs::File;

use anyhow::{Context, Result};
use futures::{StreamExt, TryStreamExt, stream};
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

struct ProductDataSelectors {
    link: Selector,
    img: Selector,
    title: Selector,
    price: Selector,
}

impl ProductDataSelectors {
    fn new() -> Self {
        Self {
            link: Selector::parse("a").unwrap(),
            img: Selector::parse("img").unwrap(),
            title: Selector::parse("h2").unwrap(),
            price: Selector::parse(".price").unwrap(),
        }
    }
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
        let a = product.select(&selectors.link).next()?;
        let img = product.select(&selectors.img).next()?;
        let h2 = product.select(&selectors.title).next()?;
        let price = product.select(&selectors.price).next()?;

        let price: String = price.text().collect();

        Some(Self {
            url: a.value().attr("href")?.to_string(),
            image: img.value().attr("src")?.to_string(),
            name: h2.text().collect(),
            price: price.replace('$', "").trim().parse().ok()?,
        })
    }
}

async fn get_product_data(
    page: usize,
    client: &reqwest::Client,
    product_selector: &Selector,
    selectors: &ProductDataSelectors,
) -> Result<Vec<ProductData>> {
    let url = format!("https://www.scrapingcourse.com/ecommerce/page/{page}");
    let response = client
        .get(&url)
        .send()
        .await
        .with_context(|| format!("request failed for {url}"))?
        .error_for_status()?
        .text()
        .await?;

    let document = Html::parse_document(&response);
    let products = document.select(product_selector);

    Ok(products
        .filter_map(|product| ProductData::get_from_element(product, selectors))
        .collect())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();

    let product_selector = Selector::parse("#product-list > li").unwrap();

    let selectors = ProductDataSelectors::new();

    let products = stream::iter(1..=12)
        .map(|page| get_product_data(page, &client, &product_selector, &selectors))
        .buffer_unordered(5)
        .try_fold(vec![], |mut acc, page_products| async {
            acc.extend(page_products);
            Ok(acc)
        })
        .await?;

    serde_json::to_writer(File::create("./data.json")?, &products)?;

    Ok(())
}
