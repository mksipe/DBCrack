
use rusqlite::{ Connection, NO_PARAMS, Result };
use std::collections::HashMap;

pub fn init_sqlite() {
    let conn = Connection::open("operation.db").unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS wordlists (path TEXT);", NO_PARAMS).unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS hashtable (ascii TEXT, md5 TEXT);", NO_PARAMS).unwrap();
}

pub fn enhance_sqlite(){
    let conn = Connection::open("operation.db").unwrap();
    conn.execute("PRAGMA auto_vacuum = FULL;", NO_PARAMS).unwrap();
    conn.execute("PRAGMA cache_size = 1000;", NO_PARAMS).unwrap();
    conn.execute("PRAGMA encoding = UTF_8;", NO_PARAMS).unwrap();
    conn.execute("PRAGMA page_size = 4096;", NO_PARAMS).unwrap();
    conn.execute("PRAGMA synchronous = FULL;", NO_PARAMS).unwrap();
    conn.execute("PRAGMA OPTIMIZE;", NO_PARAMS).unwrap();
}

pub fn insert_db(term:String) {
    let conn = Connection::open("operation.db").unwrap();
    conn.execute("INSERT INTO hashtable (ascii) VALUES (?1)", &[term]).unwrap();
}
struct wordlists {
    path: String,
}
pub fn show_wordlists() {
    let conn = Connection::open("operation.db").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM wordlists;");
    let output = stmt.query_map(NO_PARAMS, |row| {
        Ok(wordlists {
            path: row.get(0),
        })
    });
    for i in (wordlists {
        println!("{})", i);
    }
}