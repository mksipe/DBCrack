use rusqlite::{ Connection, NO_PARAMS };
use std::path::{Path};
use std::process;
use md2::{Md2, Digest};
use md4::Md4;
use md5::Md5;

fn md5() -> rusqlite::Result<()> {
        if Path::new("db.sqlite3").exists() == true {
    } else {
        println!("{}", "\nYou must create a database before you can batch one.");
        process::exit(1);
    }
        #[derive(Debug)]
    struct Wordlists {
        ASCII: String,

    }
    let stage = "MD5";
    let conn = Connection::open("db.sqlite3")?;
    let mut statement = conn.prepare("SELECT ASCII FROM hashtable")?;
    let iter = statement.query_map(NO_PARAMS, |row| {
        Ok(Wordlists{
            ASCII: row.get(0)?,
        })
    })?;
    println!("Batching: {}", stage);
    for i in iter {
        for i in i {
                let entry = i.ASCII;
                let mut hasher = Md5::new();
                hasher.update(entry.clone());
                let result = hasher.finalize();
                let result = hex::encode(result);
                let qry = "UPDATE hashtable SET MD5 = (?1) WHERE ASCII = (?2)";
                let dat = [result, entry];
                let mut stmt = conn.prepare_cached(qry)?;
                stmt.execute(&dat)?;
                conn.execute("CREATE INDEX 'MD5' ON 'hashtable' ('MD5'	ASC);", NO_PARAMS);
            }
        }
    
    Ok(())

}

fn md2() -> rusqlite::Result<()> {
        if Path::new("db.sqlite3").exists() == true {
    } else {
        println!("{}", "\nYou must create a database before you can batch one.");
        process::exit(1);
    }
        #[derive(Debug)]
    struct Wordlists {
        ASCII: String,

    }
    let stage = "MD2";
    let conn = Connection::open("db.sqlite3")?;
    conn.execute("alter table hashtable add MD2", NO_PARAMS);
    let mut statement = conn.prepare("SELECT ASCII FROM hashtable")?;
    let iter = statement.query_map(NO_PARAMS, |row| {
        Ok(Wordlists{
            ASCII: row.get(0)?,
        })
    })?;
    println!("Batching: {}", stage);
    for i in iter {
        for i in i {
                let entry = i.ASCII;
                let mut hasher = Md2::new();
                hasher.update(entry.clone());
                let result = hasher.finalize();
                let result = hex::encode(result);
                let qry = "UPDATE hashtable SET MD2 = (?1) WHERE ASCII = (?2)";
                let dat = [result, entry];
                let mut stmt = conn.prepare_cached(qry)?;
                stmt.execute(&dat)?;
                conn.execute("CREATE INDEX 'MD2' ON 'hashtable' ('MD2'	ASC);", NO_PARAMS);
            }
        }
    
    Ok(())

}
fn md4() -> rusqlite::Result<()> {
        if Path::new("db.sqlite3").exists() == true {
    } else {
        println!("{}", "\nYou must create a database before you can batch one.");
        process::exit(1);
    }
        #[derive(Debug)]
    struct Wordlists {
        ASCII: String,

    }
    let stage = "MD4";
    let conn = Connection::open("db.sqlite3")?;
    conn.execute("alter table hashtable add MD4", NO_PARAMS);
    let mut statement = conn.prepare("SELECT ASCII FROM hashtable")?;
    let iter = statement.query_map(NO_PARAMS, |row| {
        Ok(Wordlists{
            ASCII: row.get(0)?,
        })
    })?;
    println!("Batching: {}", stage);
    for i in iter {
        for i in i {
                let entry = i.ASCII;
                let mut hasher = Md4::new();
                hasher.update(entry.clone());
                let result = hasher.finalize();
                let result = hex::encode(result);
                let qry = "UPDATE hashtable SET MD4 = (?1) WHERE ASCII = (?2)";
                let dat = [result, entry];
                let mut stmt = conn.prepare_cached(qry)?;
                stmt.execute(&dat)?;
                conn.execute("CREATE INDEX 'MD4' ON 'hashtable' ('MD4'	ASC);", NO_PARAMS);
            }
        }
    
    Ok(())

}
pub fn main(){
    md2();
    md4();
    md5(); 
}