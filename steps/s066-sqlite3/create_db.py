#!/usr/bin/env python3

import os
import time
import sqlite3
import sys

def main():
	DB = "bookmarks.db"

	if os.path.exists(DB):
		print(f"Error: {DB} already exists.")
		sys.exit(1)

	conn = sqlite3.connect(DB)
	cur = conn.cursor()

	query = """
		CREATE TABLE IF NOT EXISTS Bookmark(
			ID            INT PRIMARY KEY NOT NULL,
			Title         TEXT NOT NULL,
			Path          TEXT NOT NULL,
			URL           TEXT NOT NULL,
			ADD_DATE      INT  NOT NULL,
			LAST_MODIFIED INT  NOT NULL,

			UNIQUE(Title, Path, URL)
		);
	"""
	cur.execute(query)

	now = int(time.time())

	records = [
		(1, "SQLite Home", "/databases", "https://www.sqlite.org", now, now),
		(2, "Example", "/general", "https://example.com", now, now),
	]

	query_insert = """
		INSERT INTO Bookmark
			(ID, Title, Path, URL, ADD_DATE, LAST_MODIFIED)
		VALUES (?, ?, ?, ?, ?, ?)
		ON CONFLICT(Title, Path, URL) DO NOTHING;
	"""
	cur.executemany(query_insert, records)

	query_index = """
		CREATE INDEX idx_bookmark_path ON Bookmark(Path)
	"""
	cur.execute(query_index)

	conn.commit()
	conn.close()

if __name__ == "__main__":
	main()
