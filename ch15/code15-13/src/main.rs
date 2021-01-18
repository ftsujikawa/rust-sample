use rusqlite::{params, Connection, Result};
fn main() -> Result<()> {
    let cn = Connection::open_in_memory()?;
    cn.execute("CREATE TABLE users (id INTEGER, name TEXT, age INTEGER)", params![])?;
    cn.execute("INSERT INTO users (id, name, age) VALUES (1, 'Kongo', 20)", params![])?;
    cn.execute("INSERT INTO users (id, name, age) VALUES (2, 'Hieai', 20)", params![])?;
    cn.execute("INSERT INTO users (id, name, age) VALUES (3, 'Haruna', 18)", params![])?;
    cn.execute("INSERT INTO users (id, name, age) VALUES (4, 'Kirishima', 15)", params![])?;
    Ok(())
}
