use scraper::{Html, Selector};
use std::env;
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
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("Usage: cargo run -- <url> [selector]".into());
    }

    let url = &args[1];
    let match_css = args.get(2).map(|s| s.as_str()).unwrap_or("h1, h2, h3");

    let body = fetch_page(url)?;
    let document = Html::parse_document(&body);

    let match_selector = make_selector(match_css)?;
    println!("Matches for '{}':", match_css);
    let mut match_count = 0;
    for element in document.select(&match_selector) {
        let text = element.text().collect::<String>();
        println!("  - {}", text.trim());
        match_count += 1;
    }
    println!("  ({} found)", match_count);

    let link_selector = make_selector("a")?;
    println!("\nLinks:");
    let mut link_count = 0;
    for link in document.select(&link_selector) {
        if let Some(href) = link.value().attr("href") {
            println!("  - {}", href);
            link_count += 1;
        }
    }
    println!("  ({} found)", link_count);

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
