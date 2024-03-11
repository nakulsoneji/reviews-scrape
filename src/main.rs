use review_scrape::driver;
use review_scrape::scrape;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let driver = driver::initialize_driver().await?;
    let reviews = scrape::ReviewCollection::from_amazon(
        &driver,
        "https://www.amazon.com/All-new-Echo-Show-5/product-reviews/B09B2SBHQK".to_owned(),
    )
    .await?;
    let mut review_sites = HashMap::<String, String>::new();

    reviews.write_csv("./test.csv")?;

    Ok(())
}
