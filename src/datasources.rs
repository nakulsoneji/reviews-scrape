use crate::scrape;
use thirtyfour::WebDriver;
use tokio::task::JoinSet;

pub async fn load(driver: &WebDriver) -> anyhow::Result<()> {
    std::fs::remove_dir_all("./result")?;
    std::fs::create_dir("./result")?;

    let source_file = std::fs::read_to_string("./datasources/datasources.txt")?;
    let links = source_file
        .split('\n')
        .map(|s| s.to_owned())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();
    let mut link_threads = JoinSet::new();

    for link in links {
        let reviews = scrape::ReviewCollection::from_amazon(driver, link.to_owned())
            .await
            .unwrap();
        let split_link = link
            .split('/')
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let id = split_link[split_link
            .iter()
            .position(|s| s == &"product-reviews".to_owned())
            .unwrap()
            + 1]
        .clone();

        link_threads.spawn(async move {
            reviews
                .write_csv(format!("./result/result_{}.csv", id).as_str())
                .expect("error writing csv");
        });
    }

    while link_threads.join_next().await.is_some() {}

    Ok(())
}
