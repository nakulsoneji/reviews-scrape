use thirtyfour::prelude::*;

async fn initialize_driver() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.maximize_window().await?;
    driver.goto("https://www.google.com").await?;

    Ok(())
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    initialize_driver().await?;
    Ok(())
}
