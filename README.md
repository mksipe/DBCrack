# DBCrack

## A reverse-hashing program

DISCLAIMER: This was a project just for fun and I have no intention of making this into a full-on project. In other words, I will not be maintaining it.

Warning: Does not work with Python3 very well.

Supported hash formats:
SHA1-512
MD5
NTLM
## Usage

### Adding a wordlist

This is one of the most crucial parts to this program as you need to import an external wordlist to the database.

`python DBCrack.py -w <wordlist>`

This will add the wordlist to the sqlite3 database and automatically indexes it.

### Batching the terms

This is the most time consuming parts of the program as if your using the rockyou wordlist it can take up about 8G of memory and do about 1 line every 4 seconds in the database.
I have attempted to implement some solutions to get around this, however theres no real way to do this with python as it is not a parrelelistic language and doesn't do processes asyncronously.
Currently implemented is some multithreading lines to help attempt to speed up the processes using multiple cores, however python's recent updates negated that.

`python DBCrack.py -b OK`

### Attacking hashes

To attack the hashes it takes the user input and compares it to the database.

`python DBCrack.py -a <single hash>`

`python DBCrack.py -A <hashdump>`

