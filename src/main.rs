fn main() -> Result<(), reqwest::Error> {
    let url = "https://example.com";

    let response = reqwest::blocking::get(url)?;

    println!("Status: {}", response.status());

    let body = response.text()?;

    println!("Body:\n{}", body);

    Ok(())
}
