use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let response = reqwest::Client::new()
        .get("https://www.scrapingcourse.com/ecommerce")
        .send()
        .await?
        .text()
        .await?;

    println!("{response}");

    Ok(())
}
