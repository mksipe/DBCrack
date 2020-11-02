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

#[macro_use]
extern crate clap;
use clap::App;

mod database;


fn main() {
    database::main();
    //let yaml = load_yaml!("cli.yaml");
    //let matches = App::from_yaml(yaml).get_matches();
    //This is where the actual application of the arguments take place.
    //if matches.is_present("wordlist") {
    //}
    //if matches.is_present("batch") {
    //}
    //if matches.is_present("single hash") {
    //} 
    //if matches.is_present("PWDUMP") {
    //} 
    //if matches.is_present("show_wordlist") {
    //}
}