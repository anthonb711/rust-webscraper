use std::io::{self, Write};
mod my_scrapper;
mod utils;
mod db;
mod delete;
mod insert;
mod get_archive;
mod edit;

fn main() {
  let mut user_input = String::new();
  let mut first_prompt = true;
  

  //menu loop
  loop{
    if first_prompt {
      println!("\x1b[1m-----------------------------------------------------------------------------------------------------------------\x1b[0m");
      println!("\x1b[1m|\x1b[0m\t\t\t\x1b[1mWelcome to the Church of Jesus Christ of Latter-Day Saints Newsroom scraper!\x1b[0m\t\t\x1b[1m|\x1b[0m");

      println!("\x1b[1m|\x1b[0m\t\t\t\t\t\t\t\t\t\t\t\t\t\t\x1b[1m|\x1b[0m");
      println!("\x1b[1m|\x1b[0m\t\t\t\t\t\t\t\t\t\t\t\t\t\t\x1b[1m|\x1b[0m");

      println!("\x1b[1m|\x1b[0m\t\t\t\x1b[1mMENU:\x1b[0m\t\t\t\t\t\t\t\t\t\t\t\x1b[1m|\x1b[0m");

      println!("\x1b[1m|\x1b[0m\t\t\t\t\t\t\t\t\t\t\t\t\t\t\x1b[1m|\x1b[0m");

      println!("\x1b[1m|\x1b[0m\t\t\t   'n' -> Get new news releases\t\t\t\t\t\t\t\t\x1b[1m|\x1b[0m");
      println!("\x1b[1m|\x1b[0m\t\t\t   'o' -> Get old news releases\t\t\t\t\t\t\t\t\x1b[1m|\x1b[0m");
      println!("\x1b[1m|\x1b[0m\t\t\t   'a' -> Archive new news releases\t\t\t\t\t\t\t\x1b[1m|\x1b[0m");
      println!("\x1b[1m|\x1b[0m\t\t\t   'e' -> Edit a news release\t\t\t\t\t\t\t\t\x1b[1m|\x1b[0m");
      println!("\x1b[1m|\x1b[0m\t\t\t   'd' -> Delete a news release\t\t\t\t\t\t\t\t\x1b[1m|\x1b[0m");
      println!("\x1b[1m|\x1b[0m\t\t\t   'q' -> Quit\t\t\t\t\t\t\t\t\t\t\x1b[1m|\x1b[0m");

      println!("\x1b[1m|\x1b[0m\t\t\t\t\t\t\t\t\t\t\t\t\t\t\x1b[1m|\x1b[0m");

      println!("\x1b[1m-----------------------------------------------------------------------------------------------------------------\x1b[0m");
      first_prompt = false;
    }
    print!("\x1b[1m   Enter a menu option: -> \x1b[0m" );
    io::stdout().flush().unwrap();

    user_input.clear();

    io::stdin()
    .read_line(&mut user_input)
    .expect("Error reading input.");


    match user_input.trim().chars().next() {
      Some('n') => {
       if let Err(e) = my_scrapper::scrape_url() {
        eprintln!("Scrap Error: {}", e);
       }
      },
      Some('o') => { get_archive::get_archive();              },
      Some('a') => { insert::manual_archive();                },
      Some('e') => { edit::edit_archive();                    },
      Some('d') => { delete::delete_article();                },
      Some('q') => { println!("See you next time!\n"); break; },

      _ => println!("Invaild choice"),
    }
    println!("\x1b[1m-----------------------------------------------------------------------------------------------------------------\x1b[0m");
  }
}


