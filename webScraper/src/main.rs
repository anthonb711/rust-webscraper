use std::io::{self, Write};
mod utils;



fn main() {
  let mut input = String::new();


  //menu loop
  loop{
    println!("\n\nWelcome to the Church of Jesus Christ of Latter-Day Saints Newsroom scraper.");
    println!("Any key except \"q\" will fetch new releases. \"q\" will exit.\n");
    print!("Your choice: -> " );
    io::stdout().flush().unwrap();
    io::stdin()
    .read_line(&mut input)
    .expect("Error reading input.");

    let user_input = input.trim();

    if user_input.eq_ignore_ascii_case("q"){
      // TODO: function call to fetch, parse and format data
        println!("See you next time!");
      break;
    }
    println!("\n\nHere are the three most recent new releases");
    println!("\t* 1st HEADLINE");
    println!("\t* 2nd HEADLINE");
    println!("\t* 3rd HEADLINE\n");
    break;
  }
 
}
