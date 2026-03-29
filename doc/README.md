# Bookmark Engine

This is a project for a utility and server for managing personal bookmarks.

Author: Aleksandar Janicijevic

## Introduction

This project started after a frustration: I was a user of
[Mozilla Pocket](https://en.wikipedia.org/wiki/Pocket_(service))
and I loved it. Then Mozilla discontinued and I was left looking for an
alternative. After finding a few and trying to decide which one to use,
I finally made up my mind to write my own.

- It is written in Rust programming language.
- It is open source (under [MIT License](https://github.com/ajanicij/bookmarkengine/blob/main/LICENSE).
- It provides full text search of all the bookmarked pages.

## Usage

The intended use is to export browser bookmarks and then use bookmarkengine to index
them, so that they can be searched.
Let me give you the background: after Mozilla announced discontinuing Pocket, I kept
adding bookmarks to my browser bookmarks. Then I realized, why don't I export all my
browser bookmarks and index them, so they are searchable?

Another realization I had was that I didn't just want the ability to search for
bookmarks based on their titles and descriptions - I wanted to have a full-text
seach of the content of the bookmarked pages. That makes this bookmark engine a mini
search engine. It is run locally, so you don't need to rely on a web host or cloud
provider.

## Technologies used

- Rust programming language
- Tantivy search engine
- reqwest for fetching web pages
- clap for parsing command line
- rusql for using SQLite3 database

