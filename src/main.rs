use scraper::{Html, Selector};

fn main() -> Result<(), reqwest::Error> {
    let url = "https://example.com";

    let response = reqwest::blocking::get(url)?;
    println!("Status: {}", response.status());

    let body = response.text()?;

    // Turn the raw text into a structure we can search
    let document = Html::parse_document(&body);

    // Find all headings
    let heading_selector = Selector::parse("h1").unwrap();
    println!("\nHeadings:");
    for heading in document.select(&heading_selector) {
        let text = heading.text().collect::<String>();
        println!("  - {}", text);
    }

    // Find all links
    let link_selector = Selector::parse("a").unwrap();
    println!("\nLinks:");
    for link in document.select(&link_selector) {
        if let Some(href) = link.value().attr("href") {
            println!("  - {}", href);
        }
    }

    Ok(())
}
