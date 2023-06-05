use rusqlite::{Connection, Result};
use itertools::Itertools;
use serde::{Serialize};
pub trait StringUpper{
    fn as_upper(&self) -> String;
}


impl StringUpper for String{
    fn as_upper(&self) -> String{
        let out = self.clone().to_uppercase();
        out
    }
}
impl StringUpper for str{
    fn as_upper(&self) -> String{
        let out = self.clone().to_uppercase();
        out
    }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash,Serialize)]
struct Entry {
    id: i32,
    hash: String,
    title: String,
    dt: String,
    cat: String,
    size: Option<usize>,
    ext_id: Option<String>,
    imdb: Option<String>,
}
fn load(conn: Connection) -> Result<Vec<Entry>>{
    let mut stmt = conn.prepare("SELECT id,hash,title,dt,cat,size,ext_id,imdb FROM items").unwrap();
    let person_iter = stmt.query_map([], |row| {
        Ok(Entry {
            id: row.get(0)?,
            hash: row.get(1)?,
            title: row.get(2)?,
            dt: row.get(3)?,
            cat: row.get(4)?,
            size: row.get(5)?,
            ext_id: row.get(6)?,
            imdb: row.get(7)?,
        })
    })?;

    Ok(person_iter.map(|i| i.unwrap()).collect())
}

fn search(entries: &Vec<Entry>,query: Vec<String>) -> Vec<Entry>{
    let entries = entries.clone().into_iter();
    let mut out = vec![];
    for q in query{
        let tmp = entries.clone().filter(|entry| entry.title.as_upper().contains(&q.as_upper()));
        out.append(&mut tmp.collect())
    };
    out.into_iter().unique().collect()
}
fn imdb_search(entries: &Vec<Entry>,query: String) -> Vec<Entry>{
    let entries = entries.clone().into_iter();
    let out = entries.clone().filter(|entry| entry.imdb.clone().unwrap_or("".into()).as_upper() == query.as_upper());
    out.into_iter().unique().collect()
}


fn main() {
    println!("Hello, world!");
    let conn = Connection::open("rarbg_db.sqlite").unwrap();
    let entries = load(conn).unwrap();
    println!("{}",serde_json::to_string_pretty(&search(&entries,vec!["queer".into()])).unwrap());
    println!("{}",serde_json::to_string_pretty(&imdb_search(&entries,"tt6718170".into())).unwrap());

}
