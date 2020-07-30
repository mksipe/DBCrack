pub fn banner() {
    println!("######## DBCrack - By Mason Sipe: V2.0 ########");
    println!("[1]       - Manage wordlists");
    println!("[2]       - Batch the hashlist");
    println!("[3]       - Manage an attack");
    println!("[0]       - Exit DBCrack");
    println!("########                               ########");
}

pub fn epilogue(){
    println!("######## DBCrack - Killed              ########")
}

pub fn wordlist_choices(){
    println!("######## DBCrack > Wordlist            ########");
    println!("[1]       - Add a wordlist to the database");
    println!("[2]       - Show the current added wordlists");
    println!("[0]       - Return to the previous menu");
}

pub fn wordlist_add(){
    println!("######## DBCrack > Wordlist > Add      ########");
    println!("Enter absolute path to the file.");
    println!("Type 'back' to return to previous menu.");
}

pub fn abort(){
    println!("######## DBCrack ! ABORT !             ########")
}

pub fn wordlists_loaded() {
    println!("######## DBCrack > Wordlists > Loaded  ########");
}

pub fn attack_menu() {
    println!("######## DBCrack > Attack              ########");
    println!("[1]       - Attack a single hash");
    println!("[2]       - Attack a hashdump");
    println!("[0]       - Return to menu");
}