use std::error::Error;
use std::process;
use std::fs::File;
use std::fmt;
use serde::Deserialize;
extern crate exfil;


type Record = String;

pub fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file = File::open("src/db.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    // Instead of creating an iterator with the `records` method, we create
    // an iterator with the `deserialize` method.
    for result in rdr.deserialize() {
        // We must tell Serde what type we want to deserialize into.
        let record: Record = result?;
        exfil::hash(record);
    }
    Ok(())
}
