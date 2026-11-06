# Rust Web Scraper

A small command-line program written in Rust that downloads a webpage and extracts its headings and links. Built as a beginner project to learn real-world Rust.

## What it does

Give it a URL (and optionally a CSS selector) and it prints:

1. Every element matching the selector (default: `h1, h2, h3`)
2. Every link (`href`) found on the page
3. A count of each

## Getting started

**Requirements:** [Rust and Cargo](https://rustup.rs)

```bash
git clone https://github.com/next0shi/web_scraper.git
cd web_scraper
```

## Usage

```bash
cargo run -- <url> [selector]
```

The `--` separates Cargo's options from your program's arguments.

```bash
# Default: headings (h1, h2, h3) and links
cargo run -- https://www.rust-lang.org

# Custom selector: quote text on a practice site
cargo run -- https://quotes.toscrape.com "span.text"

# Hacker News story links
cargo run -- https://news.ycombinator.com ".titleline > a"
```

Running without a URL prints a usage message.

## How it works

The program has four small functions:

| Function             | Job                                                                                                          |
| -------------------- | ------------------------------------------------------------------------------------------------------------ |
| `fetch_page(url)`    | Sends an HTTP request with `reqwest`, rejects `4xx`/`5xx` responses, returns the HTML as a `String`          |
| `make_selector(css)` | Safely turns a CSS selector string into a `Selector`, returning an error instead of crashing if it's invalid |
| `run()`              | Reads command-line arguments, fetches the page, parses the HTML, and prints matches and links                |
| `main()`             | Calls `run()` and, if it fails, prints the error and its underlying causes, then exits with code `1`         |

**Flow:** command-line arguments → HTTP request → HTML text → parsed document → CSS selector search → printed results.

## Crates used

- [`reqwest`](https://crates.io/crates/reqwest) (with the `blocking` feature): makes HTTP requests
- [`scraper`](https://crates.io/crates/scraper): parses HTML and searches it with CSS selectors

## What I learned

**Web scraping**

- An HTTP request returns a status code and a body (HTML text)
- HTML is a tree of tags; CSS selectors (`h1`, `a`, `span.text`, `h1, h2`) find parts of it
- Links store their address in the `href` attribute
- Pages that build their content with JavaScript (like `example.com` now does) show up empty to `reqwest`, because it doesn't run JavaScript

**Error handling**

- `Result` (`Ok` / `Err`) and `Option` (`Some` / `None`) make failure a normal, visible part of the code
- The `?` operator returns early with the error instead of crashing
- `map_err` converts one error type into another
- `error_for_status()` turns `404` and `500` responses into errors
- `Box<dyn Error>` is a catch-all error type that holds errors of different kinds
- `.unwrap()` crashes on failure; `.unwrap_or(default)` doesn't

**Core Rust**

- Ownership and borrowing (`&` references, why `.text()` consumes the response)
- Immutability by default, and `mut` for values that change
- Shadowing (`let response = ...` twice)
- Iterators, `for` loops, `.collect()` and the turbofish `::<String>`
- Closures (`|e| ...`), `if let`, `while let`
- `Vec`, string literals vs `String`, and macros like `println!` and `format!`
- Splitting code into small functions
- Reading command-line arguments with `std::env::args()`

## Known limitations

- Does not run JavaScript, so it only works on pages that serve their content as plain HTML
- No timeout or custom User-Agent, so some sites may block or hang on requests
- Relative links (like `/about`) are printed as they appear, not as full URLs
- Always check a site's terms of service and `robots.txt` before scraping it

## Possible improvements

1. Print the link text next to each `href`
2. Save results to a file with `std::fs::write` (or output JSON/CSV)
3. Add a timeout and custom User-Agent using `reqwest::blocking::Client::builder()`
4. Convert relative links to full URLs
5. Let the user choose the link selector too, or filter links by pattern
6. Switch to async `reqwest` with `tokio` to scrape many pages at once
7. Use a headless browser to handle JavaScript-rendered sites
8. Add unit tests for the parsing logic
9. Use the `clap` crate for proper command-line flags (`--selector`, `--output`)

## License

MIT
