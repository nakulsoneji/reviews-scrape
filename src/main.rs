use review_scrape::datasources;
use review_scrape::driver;

// TODO: add "scrapeable"/"ToCSV" trait
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let driver = driver::initialize_driver().await?;
    datasources::load(&driver).await?;
    driver.quit().await?;

    Ok(())
}
