# Overview
This project is a simple webscraper console application, written in Rust. The goal was to be able to reach a predefined URL and then scrape predefined data and display it to the user while getting my toes wet with Rust. This extends the previous version that was just scraping data and displaying it to the user. The app now archives the scraped data for later use. The user can choose to view the archived data, update, delete, or manually add data to the archive. 



[Software Demo Video](https://youtu.be/AZss8qHDkuw)

# Development Environment
- Rust with Cargo as the packet manager for the project. I did utilize the thirtyfour crate and chromedriver for webdriving.
- mysql crate is now being used extensively.
- chrono crate is being used for the date type management and formating for the database.


# Useful Websites

{Make a list of websites that you found helpful in this project}
- [Rust Offical](https://www.rust-lang.org/)
- [ThiryFour Documentation](https://vrtgs.github.io/thirtyfour/)
- [Crates - Rust Library Repo](https://crates.io/)
- [MySQL Crate Docs](https://docs.rs/mysql/latest/mysql/index.html#queryresult)

# Future Work
- Inable chromedrive to be started as part of the scraper. Currently it has to be running before the scraper is called. 
- Make it more generic to pull from different websites.