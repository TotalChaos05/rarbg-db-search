use rusqlite::{Connection, Result};
use itertools::Itertools;
use serde::{Serialize};
use std::env::args;
use std::fs::File;
use std::io::Write;
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
impl Entry{
}
fn load(conn: Connection) -> Result<Vec<Entry>>{
    let mut stmt = conn.prepare("SELECT id,hash,title,dt,cat,size,ext_id,imdb FROM items").unwrap();
    let entries = stmt.query_map([], |row| {
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
    })?.map(|i| i.unwrap()).collect();


    Ok(entries)
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

fn help(){
        println!("RARBG Database Search tool
Usage:
        rarbg-search [OPTIONS] [QUERY]
Options:
        -h --help:      show this help document
        -i --imdb:      search with imdb id
        no arguments:   plaintext search")

}

fn main() {
    let conn = Connection::open("rarbg_db.sqlite").unwrap();
    let entries = load(conn).unwrap();
    let args: Vec<String> = args().collect();
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()){
        help()
    }else if args.len() > 3 || args.len() == 1 {help()}
    else if args[1] == "-i" || args[1] == "--imdb"{
        let out: Vec<(String,String)> = imdb_search(&entries, args[2].clone()).into_iter().map(|entry| (entry.title,entry.hash)).collect();
        for entry in out{
            println!("Title:    {}",entry.0);
            println!("Link:     magnet:?xt.1={}",entry.1);
            println!("");
        }

    }else {
        let out: Vec<(String,String)> = search(&entries,args[1..].into()).into_iter().map(|entry| (entry.title,entry.hash)).collect();
        for entry in out{
            println!("Title:    {}",entry.0);
            println!("Link:     magnet:?xt=urn:btih:{}",entry.1);
            println!("");

    }

    }
}
