# DBCrack

## A reverse-hashing program

DISCLAIMER: This was a project just for fun and I have no intention of making this into a full-on project. In other words, I will not be maintaining it.

Warning: Does not work with Python3 very well.

Supported hash formats:

|Hashlib	|base	|
|:-------------:|:-----:|
|SHA1 - 512	|Base32 |
|MD5		|Base64 |
|NTLM		|       |


## Usage

### Adding a wordlist

This is one of the most crucial parts to this program as you need to import an external wordlist to the database.

`./DBCrack.py -w <wordlist>`

This will add the wordlist to the sqlite3 database and automatically indexes it.

### Batching the terms

This is the most time consuming parts of the program as if your using the rockyou wordlist it can take up about 8G of memory and do about 1 line every 4 seconds in the database.
I have attempted to implement some solutions to get around this, however theres no real way to do this with python as it is not a parrelelistic language and doesn't do processes asyncronously.
Currently implemented is some multithreading lines to help attempt to speed up the processes using multiple cores, however python's recent updates negated that.

`./DBCrack.py -b OK`

### Attacking hashes

To attack the hashes it takes the user input and compares it to the database.

`./DBCrack.py -a <single hash>`

`./DBCrack.py -A <hashdump>`

### Future ideas

Ideally I would like to move this to Python3 if possible, however, Hashlib and argparse hate python3 and don't support everything that I would like as there are a lot more hashing functions I could add if it werent for them being so difficult.
Alongside with that Argparse is the main issue currently with migrating to Python3 as it isnt very stable for reasons beyond my understanding. 
Finally I am aware of the hashlib.new function as this is where I am tring to add an additional python module to help add some more hashes to the database scalability. Such as the OpenSSL algs. 

