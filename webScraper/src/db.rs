use mysql::*;
use mysql::prelude::*;

 pub struct NewsRelease {
  id: i32,
  pub title: String,
  pub article_url: String,
  pub release_date: String,
  pub stored_date: Option<String>,
}



// Connect to the Database
pub fn db_connect() -> Result<PooledConn> {
  let database_url = "mysql://root:@127.0.0.1:3306/news_room";
  let pool = Pool::new(database_url)?;
  let conn = pool.get_conn()?;
  Ok(conn)
}


/***********************
*  CRUDE OPERATIONS 
***********************/

/************************ CREATE OPERATIONS **********************/

pub fn insert_release(title: &str, url: &str, release_date: &str) -> Result<()> {
  let mut conn = db_connect()?;

  let mysql_query = "INSERT INTO news_releases (title, article_url, release_date, stored_date) VALUES (?, ?, ?, NOW());";

  conn.exec_drop(mysql_query, (title, url, release_date,));

  Ok(())
}

/************************ READ OPERATIONS **********************/

pub fn get_all_releases() -> Result <Vec<NewsRelease>> {
  let mut conn = db_connect()?;

  let mysql_query = "SELECT * FROM news_releases;";

  let results: Vec<NewsRelease> = conn.query_map(mysql_query, |
    (id, title, article_url, release_date, stored_date) |
    { NewsRelease { id, title, article_url, release_date, stored_date,}}
  )?;

  Ok(results)
}
/************************ UPDATE OPERATIONS **********************/
/************************ DELETE OPERATIONS **********************/

