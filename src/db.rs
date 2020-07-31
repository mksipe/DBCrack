
use rusqlite::{Connection, NO_PARAMS, Result};
use std::collections::HashMap;

pub fn init_sqlite() {
    let conn = Connection::open("operation.db").unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS wordlists (path TEXT);", NO_PARAMS).unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS hashtable (ascii TEXT, calc NUMERIC, md5 TEXT);", NO_PARAMS).unwrap();
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

pub fn show_wordlists() {
    #[derive(Debug)]
    struct wordlists {
        path: String,
    }
    let conn = Connection::open("operation.db").unwrap();
    let prep = conn.prepare("SELECT * FROM wordlists");
    crate::messages::wordlist_choices();
    crate::options::option_1();
}