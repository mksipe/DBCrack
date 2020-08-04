use rusqlite::{Connection, NO_PARAMS};

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
    struct Loaded {
        path: String,
    }
    let conn = Connection::open("operation.db").unwrap();
    let mut _prep = conn.prepare("SELECT * FROM wordlists").unwrap();
    let data = _prep.query_map(NO_PARAMS, |row| Ok(Loaded {
        path: row.get_unwrap(0),
    })).unwrap();
    for i in data {
        println!("Found: {:?}", i);
    }
    crate::messages::wordlist_choices();
    crate::options::option_1();
}