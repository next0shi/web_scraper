use scraper::{Html, Selector};
use std::error::Error;

fn fetch_page(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(url)?;
    let response = response.error_for_status()?;
    let body = response.text()?;
    Ok(body)
}

fn make_selector(css: &str) -> Result<Selector, String> {
    Selector::parse(css).map_err(|e| format!("Invalid selector '{}': {}", css, e))
}

fn run() -> Result<(), Box<dyn Error>> {
    let url = "https://www.rust-lang.org";
    let body = fetch_page(url)?;
    let document = Html::parse_document(&body);

    let heading_selector = make_selector("h1, h2, h3")?;
    println!("Headings:");
    for heading in document.select(&heading_selector) {
        let text = heading.text().collect::<String>();
        println!("  - {}", text);
    }

    let link_selector = make_selector("a")?;
    println!("\nLinks:");
    for link in document.select(&link_selector) {
        if let Some(href) = link.value().attr("href") {
            println!("  - {}", href);
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);

        let mut source = e.source();
        while let Some(cause) = source {
            eprintln!("  caused by: {}", cause);
            source = cause.source();
        }

        std::process::exit(1);
    }
}
