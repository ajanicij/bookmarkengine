use rusqlite::{Connection, Result, params};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Bookmark {
    description: String,
    path: String,
    href: String,
    last_modified: u64,
}

fn create_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "create table if not exists Bookmark (
             id integer primary key,
             description text,
             path text not null,
             href text not null,
             last_modified int not null
         )",
        (),
    )?;

    Ok(())
}

fn s(text: &str) -> String {
    text.to_string()
}

fn insert_db(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    let mut bookmarks = Vec::new();
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time before UNIX_EPOCH");
    let now_epoch: u64 = duration_since_epoch.as_secs();
    bookmarks.push(
        Bookmark{
            description: s("What is Rust?"),
            path: s("/my bookmarks"),
            href: s("https://www.howtogeek.com/what-is-the-rust-programming-language-and-how-to-get-started/"),
            last_modified: now_epoch,
        }
    );

    for bookmark in &bookmarks {
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
        println!("Inserted new record: last_id={}", last_id);
    }

    tx.commit()?;

    Ok(())
}

fn query_db(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        r#"
            SELECT
                b.description, b.path, b.href, b.last_modified
            FROM
                Bookmark b;
         "#,
    )?;

    let bookmarks = stmt.query_map([], |row| {
        let last_modified: i64 = row.get(3)?;
        Ok(Bookmark {
            description: row.get(0)?,
            path: row.get(1)?,
            href: row.get(2)?,
            last_modified: last_modified as u64,
        })
    })?;

    for bookmark in bookmarks {
        if let Ok(found_bookmark) = bookmark {
            println!(
                "Found bookmark {:?}", 
                found_bookmark,
                );
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut conn = Connection::open("bookmarks.db")?;
    create_db(&conn)?;
    insert_db(&mut conn)?;
    query_db(&conn)?;
    Ok(())
}
