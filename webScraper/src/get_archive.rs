use super::db;
use super:: utils;

pub fn get_archive() {
let releases = db::get_all_releases();
      utils::print_releases(&releases);

}