#!/usr/bin/env python

import sqlite3, argparse, os, hashlib, sys, subprocess, base64, pyblake2, hmac
from sqlite3 import Error
from tqdm import tqdm

try:
	conn = sqlite3.connect('database.db')  # connect the SQLite database
except Error as e:
	print("Status: DB NOT Ready: ", e) # if not possible give an error


c = conn.cursor() # interaction with sqlite database


try:
	c.execute('''Create table "hashlist" ("ASCII" TEXT, "CALC" NUMERIC, "BASE32" TEXT, "BASE64" TEXT, "MD5" TEXT, "SHA1" TEXT, "SHA224" TEXT, "SHA256" TEXT, "SHA384" TEXT, "SHA512" TEXT, "NTLM" TEXT, "BLAKE2B" TEXT, "BLAKE2S" TEXT, "HMAC" TEXT);''')  # Create the initial schema
	c.execute('''CREATE UNIQUE INDEX "ID" ON "hashlist" ("ASCII");''') # make importing terms faster with an index which is unique to avoid duplicate terms
except:
	print("") # If it fails to do so, print nothing.





def insert_wordlist(wordlist):         # Function to add a wordlist to the database.
	if not os.path.isfile(wordlist): # if file is not found
		print("File nonexistant") # exclaim that there is no file.
		print(wordlist) # input of the user
		exit(1) # leave current shell
	else:
		f = open(wordlist, "r") # otherwise open the wordlist if it exists
	count = 0 # start a counter to determin how many words will be added starting with 0
	for line in tqdm(f): # for each word in the file along with a display of progress
		if "Undelete" in line:
			CALC = "1"
		else:
			CALC = "0"
		word = line.split("Edit")[0]
		word = word.rstrip() # sanitize input
		word = word.strip() # sanitize input
		print(word) # print added word
		try:
			c.execute('Insert INTO hashlist (ASCII, CALC) VALUES (?, ?)', (word, CALC)) # add the word to the database as a new entry.
		except:
			print(word) # print the word added
		count += 1 # add to the counter and repeat the loop
	conn.commit() # commit the actions into the database (from memory to storage)
	print(count, "records processed") # display how many entries were added
	f.close() # close the file.


def attack(string): # attack an individual string.
	try:
		c.execute('select * from hashlist where BASE32=?', (string,)) # in the column that is Base32, get the whole entry, if it matches, print it, otherise ignore it. This is persistent throughout the whole function.
		res = c.fetchall()
		res = res[0][0]
		print("BASE32  : " + string + ":" + res)
	except:
		pass
	try:
		c.execute('select * from hashlist where BASE64=?', (string,))
		res = c.fetchall()
		res = res[0][0]
		print("BASE64  : " + string + ":" + res)
	except:
		pass
	try:
		c.execute('select * from hashlist where MD5=?', (string,))
		conn.commit()
		res = c.fetchall()
		res = res[0][0]
		print("MD5: " +  string + ":" + res)
	except:
		pass
	try:
		c.execute('select * from hashlist where SHA1=?', (string,))
		conn.commit()
		res = c.fetchall()
		res = res[0][0]
		print("SHA1: " +  string + ":" + res)
	except:
		pass
	try:
		c.execute('select * from hashlist where SHA224=?', (string,))
		conn.commit()
		res = c.fetchall()
		res = res[0][0]
		print("SHA224: " +  string + ":" + res)
	except:
		pass
	try:
		c.execute('select * from hashlist where SHA256=?', (string,))
		conn.commit()
		res = c.fetchall()
		res = res[0][0]
		print("SHA256: " +  string + ":" + res)
	except:
		pass
	try:
		c.execute('select * from hashlist where SHA384=?', (string,))
		conn.commit()
		res = c.fetchall()
		res = res[0][0]
		print("SHA384: " +  string + ":" + res)
	except:
		pass

	try:
		c.execute('select * from hashlist where SHA512=?', (string,))
		conn.commit()
		res = c.fetchall()
		res = res[0][0]
		print("SHA512: " +  string + ":" + res)
	except:
		pass
	try:
		c.execute('select * from hashlist where NTLM=?', (string,))
		res = c.fetchall()
		res = res[0][0]
		print("NTLM  : " + string + ":" + res)
	except:
		pass
	try:
		c.execute('select * from hashlist where BLAKE2B=?', (string,))
		res = c.fetchall()
		res = res[0][0]
		print("BLAKE2B  : " + string + ":" + res)
	except:
		pass
	try:
		c.execute('select * from hashlist where BLAKE2S=?', (string,))
		res = c.fetchall()
		res = res[0][0]
		print("BLAKE2S  : " + string + ":" + res)
	except:
		pass
	try:
		c.execute('select * from hashlist where HMAC=?', (string,))
		res = c.fetchall()
		res = res[0][0]
		print("HMAC  : " + string + ":" + res)
	except:
		pass

def attack_list(pwdump): # use a list to attack instead of the individual hash.
	f = open(pwdump, "r") # open the file as read-only
	for i in f: # for a hash in entry
		i = i.encode("utf-8") # make sure the string is sanitized
		i = i.rstrip() # sanitize string
		i = i.strip() # sanitize string
		attack(i) # compare the strings



def batch(verify): # batching the database
	if verify != "OK": # if the user does not enter OK they are not going to run the script.
		print("Use the same command with 'OK' to verify you have enough storage.")
		raise argparse.ArgumentTypeError('')
		exit(1)

	c.execute("""SELECT DISTINCT ASCII FROM hashlist WHERE CALC='0'""") # select only one version of the term in case of duplication
	rows = c.fetchall() # get the output of the SQLite query
	count = 0 # set a counter.
	for ASCII in tqdm(rows, desc="Batching", smoothing=0.1, unit=" w"): # create a loop with a progress bar
		ASCII 		= ASCII[0] # the tupule is now a string.
		BASE32		= base64.b32encode(ASCII) # encode the string in Base32
		BASE64		= base64.b64encode(ASCII) # encode the string in Base64
		MD5 		= hashlib.md5(ASCII).hexdigest() # encode the string in MD5
		SHA1 		= hashlib.sha1(ASCII).hexdigest() # ...
		SHA224 		= hashlib.sha224(ASCII).hexdigest()
		SHA256 		= hashlib.sha256(ASCII).hexdigest()
		SHA384 		= hashlib.sha384(ASCII).hexdigest()
		SHA512 		= hashlib.sha512(ASCII).hexdigest()
		NTLM 		= hashlib.new('md4', ASCII.encode('utf-16le')).hexdigest() # encode the string in NTLM
		BLAKE2B 	= pyblake2.blake2b(ASCII.encode('utf-8')).hexdigest()
		BLAKE2S 	= pyblake2.blake2s(ASCII.encode('utf-8')).hexdigest()
		HMAC		= hmac.new(ASCII.encode('utf-8')).hexdigest()
		base32qry 	= "UPDATE hashlist SET BASE32 = ? WHERE ASCII = ?" # make a query to update the current string to have its alternative forms as an entry.
		base64qry 	= "UPDATE hashlist SET BASE64 = ? WHERE ASCII = ?" # ...
		md5qry 		= "UPDATE hashlist SET MD5 = ? WHERE ASCII = ? "
		sha1qry 	= "UPDATE hashlist SET SHA1 = ? WHERE ASCII = ? "
		sha224qry 	= "UPDATE hashlist SET SHA224 = ? WHERE ASCII = ?"
		sha256qry 	= "UPDATE hashlist SET SHA256 = ? WHERE ASCII = ?"
		sha384qry 	= "UPDATE hashlist SET SHA384 = ? WHERE ASCII = ?"
		sha512qry 	= "UPDATE hashlist SET SHA512 = ? WHERE ASCII = ?"
		NTLMqry 	= "UPDATE hashlist SET NTLM = ? WHERE ASCII = ?"
		BLAKE2Bqry 	= "UPDATE hashlist SET BLAKE2B = ? WHERE ASCII = ?"
		BLAKE2Sqry 	= "UPDATE hashlist SET BLAKE2S = ? WHERE ASCII = ?"
		HMACqry 	= "UPDATE hashlist SET HMAC = ? WHERE ASCII = ?"
		base32data      = (BASE32, ASCII) # the command that will combine both to apply the change of the entry.
		base64data      = (BASE64, ASCII) # ...
		md5data 	= (MD5, ASCII)
		sha1data 	= (SHA1, ASCII)
		sha224data 	= (SHA224, ASCII)
		sha256data 	= (SHA256, ASCII)
		sha384data 	= (SHA384, ASCII)
		sha512data 	= (SHA512, ASCII)
		ntlmdata 	= (NTLM, ASCII)
		blake2bdata = (BLAKE2B, ASCII)
		blake2sdata = (BLAKE2S, ASCII)
		hmacdata   	= (HMAC, ASCII)
		c.execute(base32qry, base32data) # execute the query.
		c.execute(base64qry, base64data) # ...
		c.execute(md5qry, md5data)
		c.execute(sha1qry, sha1data)
		c.execute(sha224qry, sha224data)
		c.execute(sha256qry, sha256data)
		c.execute(sha384qry, sha384data)
		c.execute(sha512qry, sha512data)
		c.execute(NTLMqry, ntlmdata)
		c.execute(BLAKE2Bqry, blake2bdata)
		c.execute(BLAKE2Sqry, blake2sdata)
		c.execute(HMACqry, hmacdata)
		update  = "UPDATE hashlist SET CALC=? WHERE ASCII = ? " # set the row CALC to 1 as it doesn't need to be hashed again.
		updatedata = ("1", ASCII)
		c.execute(update, updatedata)
		count +=1
		if count == 1000: # automatically commit every 1000 reps.
			conn.commit() # commit the changes.
			count= 0
	print("Indexing database ...")
	c.execute('''CREATE UNIQUE INDEX "HASHED" ON "hashlist" ("BASE32","BASE64","MD5","SHA1","SHA224","SHA256","SHA384","SHA512", "NTLM", "BLAKE2B", "BLAKE2S", "HMAC");''') # index the changes as it will make searching MUCH faster.
	conn.commit() # commit changes
	print("Done.")








def main():

	parser = argparse.ArgumentParser(usage="DBCrack.py [options]", description="Uses a database of pre-calulated hashes to make cracking faster.", prog="DBCrack.py")
	parser.add_argument("-w", 	"--wordlist"		,help="adds a wordlist to the database.", 		type=insert_wordlist, 	nargs="+")
	parser.add_argument("-b", 	"--batch"			,help="nashes all the words in the database.", 	type=batch,			 	nargs="+")
	parser.add_argument("-a", 	"--attack"			,help="compares a hash to the given database.",	type=attack,			nargs="+")
	parser.add_argument("-A", 	"--attack-list"		,help="compares a pwdump to the database",		type=attack_list,		nargs="+")
	args = parser.parse_args()

if __name__ == '__main__':
	try:
		main()
	except KeyboardInterrupt:
		print('Keybord interrupt caught')
		conn.commit()
		try:
			sys.exit(0)
		except SystemExit:
			os._exit(0)



