use review_scrape::driver;
use review_scrape::scrape;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let driver = driver::initialize_driver().await?;
    driver
        .goto("https://en.wikipedia.org/wiki/Ponzi_scheme")
        .await?;
    scrape::Review::new(driver, Some("reference".to_owned()), None, None).await?;

    Ok(())
}
