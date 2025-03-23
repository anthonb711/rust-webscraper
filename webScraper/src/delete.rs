use std::io::{self, Write};
use super::db;
use super:: utils;

pub fn delete_article() {
        println!("\n");
        let release_list =  db::get_all_releases();
        utils::print_releases(&release_list);
        
        loop{
        let mut user_input = String::new();
        print!("\x1b[1m Which article number would you like to delete? -> \x1b[0m" );
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
      let release = &release_list[article_num -1];

      let _ = db::delete_release(release.id);
      println!("Article successfuly deleted");
      break;
    }
}