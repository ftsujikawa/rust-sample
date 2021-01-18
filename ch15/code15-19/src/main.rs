use rusqlite::{params, Connection, Result};
fn main() -> Result<()> {
    let cn = Connection::open("sample.db")?;
    println!("create database.");
    Ok(())
}
