mod messages;
mod options;
mod db;
extern crate pbr;

use pbr::ProgressBar;
use std::thread;

fn main()
{
    messages::banner();
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("Enter an option: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    } else if s == "1" {
            messages::wordlist_choices();
            options::option_1();
        } else if s == "2"{
            println!("Under Development.");
        } else if s == "3" {
            messages::attack_menu();
            //actions toward selection in options to add here.
            crate::main();
        } else if s == "0" {
            messages::epilogue();
            std::process::exit(1);
        } else if s == "" {
                println!(":/");
                crate::main();
            } else {
                println!("{} did not match any valid protocols.",s);
                crate::main();
            }
        
}
