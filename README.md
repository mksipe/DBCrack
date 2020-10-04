# DBCrack 

## A Reverse-Hashing CLI Tool

---
- [DBCrack](#dbcrack)
  - [A Reverse-Hashing CLI Tool](#a-reverse-hashing-cli-tool)
  - [Usage](#usage)
    - [Adding a wordlist](#adding-a-wordlist)
    - [Show wordlists](#show-wordlists)
    - [Batching the terms](#batching-the-terms)
    - [Attacking a single hash](#attacking-a-single-hash)
    - [Attacking a PWDUMP](#attacking-a-pwdump)
  - [Software Information](#software-information)
    - [Version 2.3.3](#version-233)
    - [Requirements:](#requirements)
    - [Supported hash formats:](#supported-hash-formats)
  
---


## Usage

|Task|Full Attribute|Short Attribute|Reference|
|:-:|:-:|:-:|:-:|
|Adding a wordlist|--import-wordlist|-w|[Reference](#adding-a-wordlist)|
|Show added wordlists|--show-wordlists|-s|[Refrerence](#Show-wordlists)|
|Batching the database|--batch|-b|[Reference](#batching-the-terms)|
|Attack single hash|--attack|-a|[Reference](#attacking-a-single-hash)|
|Attack PWDUMP|--attack-file|-A|[Reference](#attacking-a-pwdump)|



### Adding a wordlist

For this program to work, you must have a list of predefined terms to be able to import into the database. The absolute path of the file will be stored in the database. Which will then be referred to in the step of batching the database. To add a wordlist to the database, you can do this with the `-w` or the `--import-wordlist` attribute to the program. If you are doing this through cargo, you can do this by typing `cargo run -- -w /usr/share/wordlists/rockyou.txt`.


### Show wordlists

This is an optional step to take if you would like to see entries that are already inside the database. To list the currently added wordlists with their ID and the number of terms detected in the file, you can use either the `-s` or the `--show-wordlists` attribute to be able to see this information. To do this through cargo, you can run `cargo run -- -s`.

### Batching the terms

Batching is a required and crucial part of this program to work correctly. Batching takes the imported directories and takes every term that is inside of the given files. Then it adds them into the hashtable. Then it will calculate the different hashes for the program to be able to use to convert the text to its ciphertext. To do this, you can either use `-b` or the `--batch` command. Through cargo, you can run `cargo run -- -b`

### Attacking a single hash

<ADD INSTRUCTIONS>

### Attacking a PWDUMP

---

## Software Information

### Version 2.3.3

### Requirements: 
The requirements for this program to work is rustc and cargo. The program will automatically parse the Cargo.toml file, which will install the dependencies from crates.io. If you, for some reason, need to install them manually, Here is the list of currently requires packages:

|Package|Version|Attributes|
|:-:|:-:|:-:|
|clap|2.33.3|yaml|
|rusqlite|0.24.0|bundled|
|md-5|0.9.1||
|hex|0.4.2||


### Supported hash formats: 

- MD5

---