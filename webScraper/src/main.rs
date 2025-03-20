use std::io::{self, Write};
mod my_scrapper;
mod db;

fn main() {
  let mut input = String::new();
  let mut first_prompt = true;

  //menu loop
  loop{
    if first_prompt {
      println!("\x1b[1m-----------------------------------------------------------------------------------------------------------------\x1b[0m");
      println!("\x1b[1m|\x1b[0m\t\t\t\x1b[1mWelcome to the Church of Jesus Christ of Latter-Day Saints Newsroom scraper!\x1b[0m            \x1b[1m|\x1b[0m");
      println!("\x1b[1m|\x1b[0m                                                                                                               \x1b[1m|\x1b[0m");
      println!("\x1b[1m|\x1b[0m\t\t\t   Quit -> ('q'/'Q')                                                                    \x1b[1m|\x1b[0m");
      println!("\x1b[1m|\x1b[0m\t\t\t   Continue -> enter any other key                                                      \x1b[1m|\x1b[0m");
      println!("\x1b[1m-----------------------------------------------------------------------------------------------------------------\x1b[0m");
      first_prompt = false;
    }
    print!("\x1b[1m   Quit or Continue: -> \x1b[0m" );
    io::stdout().flush().unwrap();

    input.clear();

    io::stdin()
    .read_line(&mut input)
    .expect("Error reading input.");
    let user_input = input.trim();

    if user_input.eq_ignore_ascii_case("q"){
      println!("See you next time!\n");
      break;
    } else if user_input.eq_ignore_ascii_case("g") {
         match db::get_all_releases() {
          Ok(release_list) =>{
            for release in release_list {
              println!(
                "*  {} {}\n URL: {}\n", release.title, release.release_date, release.article_url ); 
              }
              break;
            }

            Err(err) => eprintln!("ERROR FETCHING NEW RELEASES: {}", err),
        }
    }
    
    println!("\x1b[1m-----------------------------------------------------------------------------------------------------------------\x1b[0m");
    let _ = my_scrapper::scrape_url();

  }
}


