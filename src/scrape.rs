use thirtyfour::prelude::*;

pub struct Review {
    title: Option<String>,
    author: Option<String>,
    content: Option<String>,
}

impl Review {
    pub async fn new(
        driver: WebDriver,
        titleClass: Option<String>,
        authorClass: Option<String>,
        contentClass: Option<String>,
    ) -> WebDriverResult<Vec<Review>> {
        let mut res: Review = Review {
            title: None,
            author: None,
            content: None,
        };

        if let Some(class) = titleClass {
            let elem = driver.find(By::ClassName(class.as_str())).await?;
            println!("{}", elem.text().await?);
        }
        Ok(vec![res])
    }
}
