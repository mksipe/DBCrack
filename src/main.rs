use clap::{Arg, App};

fn main() {
    let matches = App::new("DBCrack")
        .version("2.0.1")
        .author("Mason Sipe <m-sipe@protonmail.com>")
        .about("A Reverse Lookup Password Cracker")
        .arg(Arg::with_name("wordlist")
                 .short("w")
                 .long("import-wordlist")
                 .takes_value(true)
                 .help("Import a wordlist location to the database."))
        .arg(Arg::with_name("batch")
                 .short("b")
                 .long("--batch")
                 .takes_value(false)
                 .help("First imports terms into the database, Then converts plaintext terms in the database to their hashed form."))
        .arg(Arg::with_name("single hash")
                 .short("a")
                 .long("--attack")
                 .takes_value(true)
                 .help("Attacks a single hash from the STDIN."))
        .arg(Arg::with_name("PWDUMP")
                 .short("A")
                 .long("--attack-file")
                 .takes_value(true)
                 .help("Attacks a list of hashes."))
        .get_matches();

    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", myfile);

}
