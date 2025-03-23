use std::io::{self, Write};
use super::db;
use super::utils;

pub fn edit_archive() {
    let mut input = String::new();
    let article_id: u32;

    println!("\n");
    let release_list =  db::get_all_releases();
    utils::print_releases(&release_list);


   loop{
        let mut user_input = String::new();
        print!("\x1b[1m Which article would you like to edit? -> \x1b[0m" );
        io::stdout().flush().unwrap();

        user_input.clear();

        io::stdin().read_line(&mut user_input).expect("Error reading input.");
        println!("{user_input}");

        let article_num: usize =  match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => { eprintln!("Invalid Input, please enter a article number"); return;}
      };

      if article_num == 0 || article_num > release_list.len() {
        eprintln!("That article number is not vaild");
        continue;
      }
      let article = &release_list[article_num -1];

      article_id  = article.id;
      println!("Article successfuly deleted");
      break;
    }

    // Prompt for each field
    input.clear();
    print!("New title (leave blank to keep current): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let new_title = input.trim().to_string();
    let new_title = if new_title.is_empty() { None } else { Some(new_title) };

    input.clear();
    print!("New URL (leave blank to keep current): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let new_url = input.trim().to_string();
    let new_url = if new_url.is_empty() { None } else { Some(new_url) };

    input.clear();
    print!("New release date (YYYY-MM-DD) (leave blank to keep current): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let new_date = input.trim().to_string();
    let new_date = if new_date.is_empty() { None } else { Some(new_date) };

    // Call the DB update function
    if let Err(e) = db::edit_release(
        article_id,
        new_title.as_deref(),
        new_url.as_deref(),
        new_date.as_deref(),
    ) {
        eprintln!("Failed to update: {}", e);
    }
}