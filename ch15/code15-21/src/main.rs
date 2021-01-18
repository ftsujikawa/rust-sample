use rusqlite::{params, Connection, Result};
fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let cn = Connection::open("sample.db")?;
    if args.len() <= 1 {

    }
    else {
        match args[1].as_str() {
            "init" => {
                cn.execute("CREATE TABLE users (id INTEGER, name TEXT, age INTEGER)", params![])?;
                println!("create database.");
            },
            _ => {
                println!("parameter error.");
            },
        }
    }
    Ok(())
}
