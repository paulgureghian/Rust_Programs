use rusqlite::{Connection, Result};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("notes.db")?;
    conn.execute(
        "create table if not exists notes (
            id integer primary key,
            body text not null unique
        )",
        [],
    )?;

    let mut running = true;
    while running == true {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let trimmed_body = buffer.trim();

        if trimmed_body == "" {
            running = false;
        } else {
            conn.execute("INSERT INTO notes (body) values (?1)", [trimmed_body])?;
        }
    }
    Ok(())
}
