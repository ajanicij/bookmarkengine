use rusqlite::{Connection, Result, params};
use std::collections::HashMap;


#[derive(Debug)]
struct Cat {
    name: String,
    color: String,
}

fn create_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        (),
    )?;
    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id),
             UNIQUE(name)
         )",
        (),
    )?;

    Ok(())
}

fn insert_db(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in &cat_colors {
        tx.execute(
            r#"
                INSERT INTO cat_colors (name) VALUES (?1)
                ON CONFLICT DO NOTHING
            "#,
            [color],
        )?;
        let last_id = tx.last_insert_rowid();
        println!("after INSERT INTO cat_colors: last_id={}", last_id);

        for cat in catnames {
            tx.execute(
                r#"
                    INSERT INTO cats (name, color_id) values (?1, ?2)
                    ON CONFLICT DO NOTHING
                "#,
                params![cat, last_id],
            )?;
            let last_id = tx.last_insert_rowid();
            println!("after INSERT INTO cats: last_id={}", last_id);
            }
    }

    tx.commit()?;

    Ok(())
}

fn query_db(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name FROM cats c
         INNER JOIN cat_colors cc
         ON cc.id = c.color_id;",
    )?;

    let cats = stmt.query_map([], |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    for cat in cats {
        if let Ok(found_cat) = cat {
            println!(
                "Found cat {:?} {} is {}", 
                found_cat,
                found_cat.name,
                found_cat.color,
                );
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut conn = Connection::open("cats.db")?;
    create_db(&conn)?;
    insert_db(&mut conn)?;
    query_db(&conn)?;

    Ok(())
}
