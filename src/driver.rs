use thirtyfour::prelude::*;

pub async fn initialize_driver() -> WebDriverResult<WebDriver> {
    let caps = DesiredCapabilities::firefox();
    WebDriver::new("http://localhost:4444", caps).await
}
