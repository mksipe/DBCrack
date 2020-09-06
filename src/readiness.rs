use std::path::Path;

pub fn db_exists() {
    if Path::new("db.sqlite3").exists() == true {
        println!("{}", "[ ENABLED  ] Database Ready")
    } else {
        println!("{}", "[ DISABLED ] Database Ready")
    }
}
pub fn db_scheme_exists() {
    if Path::new("db.yaml").exists() == true {
        println!("{}", "[ ENABLED  ] Database Scheme Present")
    } else {
        println!("{}", "[ DISABLED ] Database Scheme Present")
    }
}
