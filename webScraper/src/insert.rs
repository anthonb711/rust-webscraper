use std::io::{self, Write};
use super::db;
use super:: utils;

pub fn manual_archive() {
  let mut title = String::new();
  let mut url = String::new();
  let mut date = String::new();


    print!("Enter title: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut title).expect("Failed to read title");

    print!("Enter article URL: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut url).expect("Failed to read URL");

    print!("Enter release date (YYYY-MM-DD): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut date).expect("Failed to read date");

    let parsed_date= utils::parse_date(&date);

 match db::insert_release(&title.trim(), &url.trim(),  &parsed_date) {
          Ok(()) => {
            println!("Release was archived");
          }
          Err(e) => eprintln!("ERROR ARCHIVING: {}",e)
        };
}

pub fn archive_scrape(title: &str, url: &str, release_date: &str) {
  match db::insert_release(title.trim(), url.trim(), release_date.trim()) {
    Ok(()) => {},
    Err(e) => eprintln!("Error archiving scraped data: {}", e)
    }
  }
