mod readiness;
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
    readiness::db_scheme_exists();
    //This is where the actual application of the arguments take place.
    if matches.is_present("wordlist") {

    } else {

    }
    if matches.is_present("batch") {

    } else {

    }
    if matches.is_present("single hash") {
    } else {

    }
    if matches.is_present("PWDUMP") {

    } else {

    }
}