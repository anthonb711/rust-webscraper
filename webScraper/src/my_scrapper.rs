use thirtyfour;
use std::error::Error;
use thirtyfour::prelude::*;
use super::insert;
use super::utils;



// these are for the thread sleep if uncommented below
//use std::thread;
//use std::time::Duration;

#[tokio::main]
pub async fn scrape_url() -> Result<(), Box<dyn Error + Send + Sync>> {
  // webdriver configureation for chrome & headless mode
    let mut caps = DesiredCapabilities::chrome();
    caps.add_arg("--headless")?;
    caps.add_arg("--disable-gpu")?;
    caps.add_arg("--disable-dev-shm-usage")?;

    //this port must match that of the running instance of chromedriver
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    //target url is hardcoded for now
    driver.goto("https://newsroom.churchofjesuschrist.org/news-releases").await?;
    
    // maximize the window for consistency of the website rendering
    driver.maximize_window().await?;

    // create and get the desired elements 
    let release_block = driver.find(By::Id("release-block")).await?;
    let titles = release_block.find_all(By::Tag("h3")).await?;
    let links = release_block.find_all(By::Tag("a")).await?;
    let date_line = release_block.find_all(By::Css(".date-line")).await?;
    

    // format the scraped data for user readability
    println!("  Recent News Releases:\n");
    for((h3, href), date_text) in titles.iter().zip(links.iter()).zip(date_line.iter()).take(5){
      let title = h3.text().await?;
      let link = href.attr("href").await?.unwrap_or(" ".to_string());
      let date_line = date_text.find(By::Tag("span")).await?;
      let date = date_line.text().await?;
      
      //format the dateline and then archive the new releases. The db schema handles the duplicates.
      let formatted_date = utils::format_dateline(&date);
      insert::archive_scrape(&title, &link, &formatted_date);

      // display to the user
      println!("  {}", date);
      // allows for a clickable link for in some terminal applications. 
      println!("\t\x1b]8;;{}\x1b\\🔗 {} \x1b]8;;\x1b\\", link, title);  
    }


   // uncomment imports and line below if more time is needed
   // thread::sleep(Duration::from_secs(2));

    driver.quit().await?;

    Ok(())
}
