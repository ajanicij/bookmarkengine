use rusqlite::{Connection, Result, params, Row};
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Bookmark {
    pub description: Option<String>,
    pub path: String,
    pub href: String,
    pub last_modified: i64,
}

pub struct BookmarkDb {
    pub conn: Connection,
}

impl Bookmark {
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Bookmark {
            description: row.get("description")?,
            path: row.get("path")?,
            href: row.get("href")?,
            last_modified: row.get("last_modified")?,
        })
    }
}

impl BookmarkDb {
    pub fn new(db: &PathBuf) -> Result<BookmarkDb> {
        let conn = Connection::open(db)?;
        Ok(BookmarkDb{ conn })
    }

    pub fn create_db(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS Bookmark (
                 id INTEGER PRIMARY KEY,
                 description TEXT,
                 path TEXT NOT NULL,
                 href TEXT NOT NULL,
                 last_modified INTEGER NOT NULL,
                 UNIQUE(path, href)
             )",
            (),
        )?;
    
        Ok(())
    }

    pub fn insert(&mut self, bookmark: &Bookmark) -> Result<i64, Box<dyn Error>> {
        let tx = self.conn.transaction()?;

        tx.execute(
            r#"
                INSERT INTO Bookmark 
                    (description, path, href, last_modified)
                VALUES
                    (?, ?, ?, ?)
                ;
            "#,
            params![
                bookmark.description,
                bookmark.path,
                bookmark.href,
                bookmark.last_modified as i64,
            ]
        )?;
        let last_id = tx.last_insert_rowid();

        tx.commit()?;

        Ok(last_id)
    }

    pub fn delete(&mut self, bookmark: &Bookmark) -> Result<(), Box<dyn Error>> {
        let tx = self.conn.transaction()?;

        tx.execute(
            r#"
                DELETE FROM Bookmark
                WHERE
                    path = ? AND href = ?
                ;
            "#,
            params![
                bookmark.path,
                bookmark.href,
            ]
        )?;

        tx.commit()?;

        Ok(())
    }

    pub fn search(&mut self, bookmark: &Bookmark) -> Result<Vec<i64>, Box<dyn Error>> {
        let mut stmt = self.conn.prepare(
            r#"
                SELECT
                    id
                FROM
                    Bookmark
                WHERE
                    path = ?1
                    AND href = ?2
            "#
        )?;

        let rows = stmt.query_map(
            params![
                bookmark.path,
                bookmark.href,
            ],
            |row| {
                row.get::<_, i64>(0)
            }
        )?;

        let mut ids = Vec::new();
    
        for id in rows {
            ids.push(id?);
        }

        Ok(ids)
    }

    pub fn exists(&mut self, bookmark: &Bookmark) -> Result<bool, Box<dyn Error>> {
        let ids = self.search(bookmark)?;
        Ok(ids.len() > 0)
    }

    pub fn load_all(&self) -> rusqlite::Result<Vec<Bookmark>> {
        let mut stmt = self.conn.prepare(
            "SELECT description, path, href, last_modified FROM Bookmark"
        )?;
    
        let rows = stmt.query_map([], Bookmark::from_row)?;
    
        rows.collect()
    }
}
