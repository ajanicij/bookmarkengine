#!/usr/bin/env python3

import os
import sqlite3

def main():
	DB = "bookmarks.db"

	if not os.path.exists(DB):
		print("Error: database not found.")
		sys.exit(1)

	conn = sqlite3.connect(DB)
	cur = conn.cursor()

	query = "SELECT * FROM Bookmark"
	cur.execute(query)

	rows = cur.fetchall()

	for (id, title, path, url, add_date, last_modified) in rows:
		print(f"{id} | {title} | {path} | {url} | {add_date} | {last_modified}")

	conn.close()

if __name__ == "__main__":
	main()
