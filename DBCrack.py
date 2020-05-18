import sqlite3, argparse, os, hashlib
conn = None

try:
	conn = sqlite3.connect('database.db')
except Error as e:
	print("Status: DB NOT Ready: ", e)



c = conn.cursor()

try:
	c.execute('''Create table "hashlist"("ASCII" TEXT, "Deleted" TEXT, "MD5" TEXT, "SHA1" TEXT, "SHA256" TEXT, "SHA512" TEXT);''')
except:
	print("")



def insert_wordlist(wordlist):
	if not os.path.isfile(wordlist):
		print("File nonexistant")
		print(wordlist)
		exit(1)
	else:
		f = open(wordlist, "r")
	count = 0
	for line in f:
		if "Undelete" in line:
			Deleted = "Y"
		else:
			Deleted = "N"
		word = line.split("Edit")[0]
		word = word.rstrip()
		word = word.strip()
		print(word)
		try:
			c.execute('Insert INTO hashlist (ASCII, Deleted) VALUES (?, ?)', (word, Deleted))
		except:
			print("A duplicate record was found:", word)
		count += 1
	conn.commit()
	print(count, "records processed")
	f.close()


def attack(string):

	try:
		c.execute('select * from hashlist where MD5=?', (string,))
		conn.commit()
		res = c.fetchall()
		res = res[0][0]
		print("MD5: ",  string, " | ", res)
	except:
		print("MD5: Not found.")
	try:
		c.execute('select * from hashlist where SHA1=?', (string,))
		conn.commit()
		res = c.fetchall()
		res = res[0][0]
		print("SHA1: ",  string, " | ", res)
	except:
		print("SHA1: Not found.")
	try:
		c.execute('select * from hashlist where SHA256=?', (string,))
		conn.commit()
		res = c.fetchall()
		res = res[0][0]
		print("SHA256: ",  string, " | ", res)
	except:
		print("SHA256: Not found.")
	try:
		c.execute('select * from hashlist where SHA512=?', (string,))
		conn.commit()
		res = c.fetchall()
		res = res[0][0]
		print("SHA512: ",  string, " | ", res)
	except:
		print("SHA512: Not found.")


def attack_list(pwdump):
	f = open(pwdump, "r")
	for i in f:
		i = i.encode("utf-8")
		i = i.rstrip()
		i = i.strip()
		attack(i)



def batch(verify):
	if verify != "OK":
		print("Use the same command with 'OK' to verify you have enough storage.")
		exit(1)
	c.execute("""SELECT ASCII FROM hashlist""")
	rows = c.fetchall()
	for ASCII in rows:
		ASCII = ASCII[0]
		ASCII = ASCII.encode('utf-8')
		print("Hashing: " + ASCII)
		MD5 = hashlib.md5(ASCII).hexdigest()
		SHA1 = hashlib.sha1(ASCII).hexdigest()
		SHA256 = hashlib.sha256(ASCII).hexdigest()
		SHA512 = hashlib.sha512(ASCII).hexdigest()
		md5qry = "UPDATE hashlist SET MD5 = ? WHERE ASCII = ? "
		sha1qry = "UPDATE hashlist SET SHA1 = ? WHERE ASCII = ? "
		sha256qry = "UPDATE hashlist SET SHA256 = ? WHERE ASCII = ?"
		sha512qry = "UPDATE hashlist SET SHA512 = ? WHERE ASCII = ?"
		md5data = (MD5, ASCII)
		sha1data = (SHA1, ASCII)
		sha256data = (SHA256, ASCII)
		sha512data = (SHA512, ASCII)
		c.execute(md5qry, md5data)
		c.execute(sha1qry, sha1data)
		c.execute(sha256qry, sha256data)
		c.execute(sha512qry, sha512data)
		conn.commit()

def main():
	parser= argparse.ArgumentParser(usage="DBCrack.py [options]", description="Uses a database of pre-calulated hashes to make cracking faster.", prog="DBCrack.py")
	parser.add_argument("-w", "--wordlist"		,help="adds a wordlist to the database.", 		type=insert_wordlist)
	parser.add_argument("-b", "--batch"			,help="nashes all the words in the database.", 	type=batch)
	parser.add_argument("-a", "--attack"		,help="compares a hash to the given database.",	type=attack)
	parser.add_argument("-A", "--attack-list"	,help="compares a pwdump to the database",		type=attack_list)
	args = parser.parse_args()

main()


