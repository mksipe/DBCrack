use std::path::Path;

pub fn db_exists() {
    if Path::new("db.sqlite3").exists() == true {
        println!("{}", "[ ENABLED  ] Database Ready")
    } else {
        println!("{}", "[ DISABLED ] Database Ready")
    }
}
