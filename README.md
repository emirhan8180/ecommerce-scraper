# E-Commerce Scraper

A web scraper to extract product data from [this site](https://www.scrapingcourse.com/ecommerce).

## Installation

Make sure that Rust is installed, and then clone the repo:

```bash
git clone https://github.com/emirhan8180/ecommerce-scraper.git
cd ecommerce-scraper
```

## Usage

Simply run the following, and it will output the scraped data to `./data.json`:

```bash
cargo run
```

## Features

* Getting page content with [reqwest](https://docs.rs/reqwest/latest/reqwest)
* HTML parsing with [scraper](https://docs.rs/scraper/latest/scraper)
* Parallel data fetching with [tokio](https://docs.rs/tokio/latest/tokio) and [futures](https://docs.rs/futures/latest/futures)
* JSON convertion with [serde](https://docs.rs/serde/latest/serde)
