#!/usr/bin/env bash

# Exit immediately on any error.
set -e

echo "Creating bookmarks.db"
python3 create_db.py

echo "Dumping bookmarks.db"
python3 dump_db.py

echo "Done"
