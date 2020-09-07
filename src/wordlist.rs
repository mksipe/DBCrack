use rusqlite::{ Connection, NO_PARAMS };
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::convert::TryInto;


fn init() {
    let conn = Connection::open("db.sqlite3").unwrap();
    conn.execute("CREATE TABLE hashtable(
        id INTERGER PRIMARY KEY,
        ASCII TEXT,
        MD5 TEXT
    )", NO_PARAMS).unwrap();
    conn.execute("CREATE TABLE wordlists(
        id INTERGER PRIMARY KEY,
        PATH TEXT,
        WORDS INTERGER
    )", NO_PARAMS).unwrap();

    // let ASCII: String = "Steve Example".to_string();
    // let MD5: String = "steve@example.org".to_string();  
    // conn.execute("INSERT INTO person (name, email) VALUES (?1, ?2)",
    //      &[&name, &email]).unwrap();

}

fn CheckDB() {
        if Path::new("db.sqlite3").exists() == true {
    } else {
        init()
    }
}

pub fn main(){
    CheckDB()
    
}
static mut count: i64 = 0;

pub fn add(value: &str) {
    let conn = Connection::open("db.sqlite3").unwrap();
    let file = File::open(value).unwrap();
    let buff = BufReader::new(file);
    for (index, line) in buff.lines().enumerate(){
        let line = line.unwrap();
        unsafe{count = ((index + 1)).try_into().unwrap();}
    }
    unsafe {
        let countstr = count.to_string();
        let counter: &str = &countstr;
        conn.execute("INSERT INTO wordlists(PATH, WORDS) VALUES (?1, ?2)", &[&value, &counter]).unwrap();
    }
}