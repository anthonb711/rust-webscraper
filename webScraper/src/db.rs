use mysql::*;
use mysql::prelude::*;

 pub struct NewsRelease {
  pub id: u32,
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
   let _ = conn.exec_drop(mysql_query, (title, url, release_date,));
  Ok(())
}

/************************ READ OPERATIONS **********************/

pub fn get_all_releases() -> Vec<NewsRelease> {
  let mut conn = match db_connect() {
    Ok(c) => c,
    Err(e) => {
      eprintln!("Error connecting to News Release DB: {}", e);
      return vec![];
    }
  };

  let mysql_query = "SELECT * FROM news_releases;";

  match conn.query_map(mysql_query, |
    (id, title, article_url, release_date, stored_date,) |
    { NewsRelease {id, title, article_url, release_date, stored_date,}}
  ) {
    Ok(list) => list,
    Err(e) => {
      eprintln!("ERROR FETCHING NEW RELEASES: {}", e);
      vec![]
    }
  }
}
/************************ UPDATE OPERATIONS **********************/
pub fn edit_release(
  id:        u32,
  new_title: Option<&str>,
  new_url:   Option<&str>,
  new_date:  Option<&str>,

) -> Result<()> {
   let mut conn = match db_connect() {
    Ok(c) => c,
    Err(e) => {
      eprintln!("Error connecting to News Release DB: {}", e);
      return Err(e.into());
    }
  };

  let mut article_columns = vec![];
  let mut params: Vec<Value> =vec![];

  if let Some(title) = new_title {
        article_columns.push("title = ?");
        params.push(title.into());
    }
  if let Some(url) = new_url {
        article_columns.push("article_url = ?");
        params.push(url.into());
    }
  if let Some(date) = new_date {
        article_columns.push("release_date = ?");
        params.push(date.into());
    }
  if article_columns.is_empty() {
        println!("No columns provided to update.");
        return Ok(());
    }

  let mysql_query = format!( "UPDATE news_releases SET {}  WHERE news_releases_id = ?",
    article_columns.join(", ")
  );

  params.push(id.into());

println!("THE IS THE EDIT QUERY: {}", mysql_query);
 if let Err(e) = conn.exec_drop(mysql_query, params) {
     eprintln!("Error updating archive: {}", e);
     return Err(e.into());
  }
  Ok(())
}

/************************ DELETE OPERATIONS **********************/
pub fn delete_release(id: u32) -> Result<()> {
   let mut conn = match db_connect() {
    Ok(c) => c,
    Err(e) => {
      eprintln!("Error connecting to News Release DB: {}", e);
      return Err(e.into());
    }
  };

  let mysql_query = "DELETE FROM news_releases WHERE news_releases_id = ?";

 if let Err(e) = conn.exec_drop(mysql_query, (id,)) {
     eprintln!("Error Deleteing article: {}", e);
     return Err(e.into());
  }
  Ok(())
}

