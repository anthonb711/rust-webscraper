use super::db;
use chrono::NaiveDate;

// Format release info from db
pub fn print_releases(release_list: &Vec<db::NewsRelease>) {

  let mut count  = 0;

  for release in release_list {
            count += 1;
              println!(
                "{count} : {} : {}\n\t URL: {}\n", release.release_date, release.title, release.article_url ); 
  }
}

pub fn parse_date(date_input: &str) -> String{
   match NaiveDate::parse_from_str(date_input.trim(), "%Y-%m-%d") {
    Ok(date) => date.format("%Y-%m-%d").to_string(),
    Err(e) => {
      eprintln!("Invalid date format: {}", e);
      String::new()
    }
  }
}

pub fn format_dateline(dateline: &str) -> String{
  let date_slice = dateline.split('|').next().unwrap().trim();

  match NaiveDate::parse_from_str(date_slice,"%d %B %Y") {
    Ok(date) => date.format("%y-%m-%d").to_string(),
    Err(_) =>   String::new(),
  }
}