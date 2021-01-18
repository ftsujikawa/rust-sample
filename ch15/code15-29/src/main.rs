use rusqlite::{params, Connection, Result};
use chrono::prelude::*;
fn main() -> Result<()> {
    let cn = Connection::open("sample.db")?;
    let mut stmt = cn.prepare("
        SELECT
            T品名.name,
            T販売方法.name,
            t.卸売数
        FROM T卸売 t
        inner join T品名 on t.品名id = T品名.id
        inner join T販売方法 on t.販売方法id = T販売方法.id
        inner join T市場 on t.市場id = T市場.id
        where t.日付 = '2020-01-12'
          and T市場.name = '築地'
        order by t.卸売数 desc
    ")?;
    let mut rows = stmt.query(params![])?;
    while let Some(row) = rows.next()? {
        let name: String = row.get(0)?;
        let sale: String = row.get(1)?;
        let sum: i32 = row.get(2)?;
        println!("{} {} 卸売数:{}", name, sale, sum);
    }
    Ok(())
}
