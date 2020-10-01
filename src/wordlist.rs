use rusqlite::{ Connection, NO_PARAMS };
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::convert::TryInto;

//use std::io::Read;
//use std::ffi::{OsString, OsStr};


fn init() {
    let conn = Connection::open("db.sqlite3").unwrap();
    conn.execute("CREATE TABLE hashtable (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        ASCII text NOT NULL,
        MD5 text NOT NULL
    )", NO_PARAMS).unwrap();
    conn.execute("CREATE TABLE wordlists (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        PATH text NOT NULL,
        WORDS text NOT NULL
    )", NO_PARAMS).unwrap();

    // let ASCII: String = "Steve Example".to_string();
    // let MD5: String = "steve@example.org".to_string();  
    // conn.execute("INSERT INTO person (name, email) VALUES (?1, ?2)",
    //      &[&name, &email]).unwrap();

}

fn check_db() {
        if Path::new("db.sqlite3").exists() == true {
    } else {
        init()
    }
}

pub fn main(){
    check_db()
    
}


pub fn add(value: &str) {
    let mut count: i64 = 0;
    let conn = Connection::open("db.sqlite3").unwrap();
    let file = File::open(value).unwrap();
    let buff = BufReader::new(&file);
    for (index, _line) in buff.lines().enumerate(){
        let _ifile = File::open(value).expect("Unable to open file");
        count = ((index + 1)).try_into().unwrap();
    }
    
    let countstr = count.to_string();
    let counter: &str = &countstr;
    conn.execute("INSERT INTO wordlists(PATH, WORDS) VALUES (?1, ?2)", &[&value, &counter]).unwrap();
    conn.execute("delete from wordlists where rowid NOT IN (SELECT MIN(rowid) from wordlists group by PATH)", NO_PARAMS).unwrap(); // remove duplicate paths.
    
}



pub fn show_wordlists() -> rusqlite::Result<()> {
    check_db();
    #[derive(Debug)]
    struct Wordlists {
        id: i32,
        name: String,
        words: String,
    }
    let conn = Connection::open("db.sqlite3")?;
    let mut statement = conn.prepare("SELECT * FROM wordlists")?;
    let iter = statement.query_map(NO_PARAMS, |row| {
        Ok(Wordlists{
            id: row.get(0)?,
            name: row.get(1)?,
            words: row.get(2)?,
        })
    })?;
    println!("{}", "Table   |   Words:Path");
    for i in iter {
        for i in i {
            println!("{:?}       |    {:?}:{:?}", i.id, i.words, i.name);
        }
    }
    Ok(())

}