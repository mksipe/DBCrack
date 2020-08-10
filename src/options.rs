use std::path::Path;
use std::io::Read;
fn home() {
    super::main();
}

fn add_wordlist () {
    crate::messages::wordlist_add();
    crate::db::init_sqlite();
    let _s = "";
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("File: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    if Path::new(&s).exists() == false {
        println!("ERR: The file {} does not appear to exist.", s);
        if s == "back"{
            crate::messages::abort();
            super::main();
        }
        crate::options::add_wordlist();
    } else if Path::new(&s).exists() == true {
        println!("\n File {} locked. Retrieving contents ...", s);
        let mut file = std::fs::File::open(&s).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        let split = contents.split("\n");
        for i in split {
            crate::db::insert_db(i.to_string());
        }
        crate::db::enhance_sqlite();
        crate::options::option_1();
    } 
}

pub fn option_1() {
    println!("Enter an option: ");
    crate::db::init_sqlite();
    let _s = "";
    use std::io::{stdin,stdout};
    let mut s=String::new();
    let _=stdout();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    } 
    if s == "1" {
            add_wordlist();
        } else if s == "2" {
            crate::messages::wordlists_loaded();
            crate::db::show_wordlists();
            crate::options::option_1();
            } else if s == "0" {
                home();
            } else if s == "" {
                println!(":/");
                option_1();
            } else {
                println!("{} did not match any valid protocols.",s);
                crate::main();
            }
}