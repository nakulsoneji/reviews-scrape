use review_scrape::driver;
use review_scrape::scrape;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let driver = driver::initialize_driver().await?;
    let reviews = scrape::ReviewCollection::from_amazon(
        &driver,
        "https://www.amazon.com/All-new-Echo-Show-5/product-reviews/B09B2SBHQK".to_owned(),
    )
    .await?;

    reviews.print();

    driver.quit().await?;
    Ok(())
}
