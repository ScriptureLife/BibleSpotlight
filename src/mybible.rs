use rusqlite::{params, Connection, Result};

use std::fs;
use std::path::PathBuf;
use std::process::exit;
use dirs;
use crate::BibleReference;


fn find_version(version: &str) -> Option<String> {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from(""));
    let dir = home_dir.join(".config/bible-spotlight/bibles");

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.contains(version) {
                        return Some(entry.path().to_str().unwrap().to_string());
                    }
                }
            }
        }
    }
    None
}

pub fn main(bible_reference: Option<BibleReference>, version: &str) -> Result<String, rusqlite::Error> {
    
    let conn: Connection;


    let path_version = find_version(version);
    if path_version.is_some() {
        conn = Connection::open(path_version.unwrap())?;
    } else {

        eprintln!("Could not find version: \"{}\"", version);
        exit(1)
        
    }



    let fetch_book = format!("SELECT book_number FROM books WHERE long_name LIKE \"{}\"", bible_reference.as_ref().unwrap().book.trim_end());
    // println!("{:?}", &fetch_book);
    let book_number: u32 = conn.query_row(&fetch_book, params![], |row| row.get(0))?;
   
    let fetch_verse: String;


    if bible_reference.as_ref().unwrap().verse.last().is_none() {
        fetch_verse = format!("SELECT text FROM verses WHERE book_number = {} AND chapter = {}", book_number, bible_reference.as_ref().unwrap().chapter[0]);
    } else {
        fetch_verse = format!("SELECT text FROM verses WHERE book_number = {} AND chapter = {} AND verse BETWEEN {} AND {}", book_number, bible_reference.as_ref().unwrap().chapter[0], bible_reference.as_ref().unwrap().verse[0], bible_reference.as_ref().unwrap().verse.last().unwrap());
    }

    // println!("{:?}", &fetch_verse);

    let mut stmt = conn.prepare(&fetch_verse)?;
    let verse_iter = stmt.query_map(params![], |row| {Ok(row.get::<_, String>(0)?)
    })?;

    let mut verses = String::from("");
    for verse in verse_iter {
        if verses == "" {
            verses = format!{"{}", verse.unwrap().to_string()};
            continue;
        }
        verses = format!{"{} {}", verses, verse.unwrap().to_string()};
    }



    return Ok(verses)

}