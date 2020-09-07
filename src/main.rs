//   Copyright (C) 2020  Mason Sipe <m-sipe@protonmail.com>
//
//    This program is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, either version 3 of the License, or
//    (at your option) any later version.
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <https://www.gnu.org/licenses/>.


mod readiness;
mod wordlist;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    println!("{}", "\nArguments Selected:\n");
    // This will check to see what is already selected to be displayed. 
    if matches.is_present("wordlist") {
        println!("{}", "[ ENABLED  ] Wordlist Addition");
    } else {
        println!("{}", "[ DISABLED ] Wordlist Addition")
    }
    if matches.is_present("batch") {
        println!("{}", "[ ENABLED  ] Batch Database");
    } else {
        println!("{}", "[ DISABLED ] Batch Database")
    }
    if matches.is_present("single hash") {
        println!("{}", "[ ENABLED  ] Single Hash Attack");
    } else {
        println!("{}", "[ DISABLED ] Single Hash Attack")
    }
    if matches.is_present("PWDUMP") {
        println!("{}", "[ ENABLED  ] PWDUMP Attack");
    } else {
        println!("{}", "[ DISABLED ] PWDUMP Attack")
    }
    println!("{}", "\nReadiness Information:\n");
    readiness::db_exists();
    //This is where the actual application of the arguments take place.
    if matches.is_present("wordlist") {
        wordlist::main();
        let file: &str  = matches.value_of("wordlist").unwrap();
        wordlist::add(file);

    }
    if matches.is_present("batch") {
    }
    if matches.is_present("single hash") {
    } 
    if matches.is_present("PWDUMP") {

    } 
}