use rusqlite::{params, Connection, Result};
use chrono::prelude::*;
#[derive(Debug)]
struct Fish {
    id: i32,
    date: NaiveDate,
    num: i32,
    name: String,
    sale: String,
    market: String,
}
fn main() -> Result<()> {
    let cn = Connection::open("sample.db")?;
    let mut stmt = cn.prepare("
        SELECT
            t.id,
            t,日付,
            t.卸売数,
            T品名.name,
            T販売方法.name,
            T市場.name
        FROM T卸売 t
        inner join T品名 on T.品名id = T品名.id
        inner join T販売方法 on t.販売方法id = T販売方法.id
        inner join T市場 on t.市場id = T市場.id
        where t.日付 = '2020-01-12'
    ")?;
    let iter = stmt.query_map(params![], |row| {
        let dt: String = row.get(1)?;
        Ok(Fish {
            id: row.get(0)?,
            date: NaiveDate::parse_from_str(&dt, "%Y-%m-%d").unwrap(),
            num: row.get(2)?,
            name: row.get(3)?,
            sale: row.get(4)?,
            market: row.get(5)?,
        })
    })?;
    for it in iter {
        println!("{:?}", it.unwrap());
    }
    Ok(())
}
