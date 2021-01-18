use rusqlite::{params, Connection, Result};
fn main() -> Result<()> {
    let cn = Connection::open_in_memory()?;
    cn.execute_batch (
        "
        CREATE TABLE users (id INTEGER, name TEXT, age INTEGER);
        INSERT INTO users (id, name, age) VALUES (1, 'Kongo', 20);
        INSERT INTO users (id, name, age) VALUES (2, 'Hieai', 20);
        INSERT INTO users (id, name, age) VALUES (3, 'Haruna', 18);
        INSERT INTO users (id, name, age) VALUES (4, 'Kirishima', 15);
        "
    )?;
    Ok(())
}
