#!/usr/bin/env python3
import hashlib

def get_hash(file_path):
	myhash = hashlib.sha256()
	with open(file_path,'rb') as file:
		content = file.read()
		myhash.update(content)
		rv = myhash.hexdigest()
		return rv

if __name__ == '__main__':
	filepath = "testfile"
	get_hash(filepath)