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
            "into" => {
                let id: i32 = args[2].parse::<i32>().unwrap();
                let name = &args[3];
                let age: i32 = args[4].parse::<i32>().unwrap();
                let mut stmt = cn.prepare("INSERT INTO users (id, name, age) VALUES (?, ?, ?)")?;
                stmt.execute(params![id, name, age])?;
                println!("insert data.");
            },
            _ => {
                println!("parameter error.");
            },
        }
    }
    Ok(())
}
