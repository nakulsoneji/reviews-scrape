use serde::{Deserialize, Serialize};
use std::fs::File;
use thirtyfour::{components::ElementResolver, prelude::*};
use thirtyfour_macros::*;

const MAX_PAGES: i32 = 10;

#[derive(Serialize, Deserialize)]
pub struct Review {
    author: String,
    title: String,
    content: String,
}

pub struct ReviewCollection {
    reviews: Vec<Review>,
}

#[derive(Debug, Clone, Component)]
pub struct AmazonReview {
    base: WebElement,
    #[by(
        xpath = ".//div[@class='a-row a-spacing-none']/div[@class='a-section celwidget']/div[@class='a-row']"
    )]
    title: ElementResolver<WebElement>,
    #[by(
        xpath = ".//div[@class='a-row a-spacing-none']/div[@class='a-section celwidget']/div[@class='a-row a-spacing-mini']"
    )]
    author: ElementResolver<WebElement>,
    #[by(
        xpath = ".//div[@class='a-row a-spacing-none']/div[@class='a-section celwidget']/div[@class='a-row a-spacing-small review-data']"
    )]
    content: ElementResolver<WebElement>,
}

impl ReviewCollection {
    pub async fn from_amazon(driver: &WebDriver, link: String) -> anyhow::Result<ReviewCollection> {
        driver.goto(link.as_str()).await?;

        let count_elem = driver
            .find(By::XPath(
                "//div[@data-hook='cr-filter-info-review-rating-count']",
            ))
            .await?;
        let count_reviews = count_elem
            .text()
            .await?
            .split(' ')
            .nth(3)
            .unwrap()
            .replace(',', "")
            .parse::<f32>()
            .unwrap();
        let mut total_pages = (count_reviews / 10.0).ceil() as i32;

        if total_pages > MAX_PAGES {
            total_pages = MAX_PAGES;
        }

        let mut reviews: Vec<Review> = vec![];

        for i in 0..total_pages {
            if i != 0 {
                driver
                    .goto(format!("{}?pageNumber={}", link, i + 1))
                    .await?;
            }
            let elems = driver.find_all(By::ClassName("review")).await?;
            for e in elems {
                let mut review = Review::from(AmazonReview::from(e)).await?;
                review.content = review.content.replace('\n', "");
                reviews.push(review);
            }
        }

        Ok(ReviewCollection { reviews })
    }
    pub fn print(&self) {
        let mut i = 1;
        for r in self.reviews.iter() {
            println!(
                "{}.\ntitle: {}\nauthor: {}\ncontent: {}\n",
                i, r.title, r.author, r.content
            );
            i += 1;
        }
    }
    pub fn write_csv(&self, file_path: &str) -> anyhow::Result<()> {
        let _ = File::create(file_path)?;

        let mut wtr = csv::Writer::from_path(file_path)?;

        for review in self.reviews.iter() {
            wtr.serialize(review)?;
        }
        Ok(())
    }
}

impl Review {
    pub async fn from(review: AmazonReview) -> anyhow::Result<Review> {
        Ok(Review {
            title: review.title.resolve().await?.text().await?,
            author: review.author.resolve().await?.text().await?,
            content: review.content.resolve().await?.text().await?,
        })
    }
}
